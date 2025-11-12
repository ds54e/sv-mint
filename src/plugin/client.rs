use crate::config::Config;
use crate::core::errors::PluginError;
use crate::core::payload::StagePayload;
use crate::diag::event::{Ev, Event};
use crate::diag::logging::log_event;
use crate::plugin_scripts::{resolve_script_path, resolve_scripts};
use crate::types::{Severity, Stage, Violation};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::path::Path;
use std::process::Stdio;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use tokio::io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader};
use tokio::process::{Child, ChildStderr, ChildStdin, ChildStdout, Command};
use tokio::runtime::Runtime;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use tokio::time;

pub struct PythonHost {
    runtime: Runtime,
    child: Child,
    stdin: ChildStdin,
    stdout_rx: UnboundedReceiver<String>,
    stderr_buf: Arc<Mutex<Vec<u8>>>,
    stderr_pos: usize,
    timeout: Duration,
    snippet_limit: usize,
    severity_override: HashMap<String, Severity>,
}

#[derive(Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
enum HostRequest<'a> {
    Init {
        scripts: &'a [String],
    },
    RunStage {
        stage: &'a str,
        path: &'a Path,
        payload: Value,
    },
    Shutdown,
}

#[derive(Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
enum HostResponse {
    Ready,
    Violations { violations: Vec<Violation> },
    Error { detail: Option<String> },
}

impl PythonHost {
    pub fn start(cfg: &Config) -> Result<Self, PluginError> {
        let scripts = resolve_scripts(cfg);
        let host_path = resolve_script_path("plugins/lib/rule_host.py");
        let runtime = Runtime::new().map_err(|e| PluginError::SpawnFailed { detail: e.to_string() })?;
        let timeout = Duration::from_millis(cfg.defaults.timeout_ms_per_file);
        let snippet_limit = cfg.logging.stderr_snippet_bytes;
        let severity_override = build_severity_override(&cfg.ruleset.severity_override);
        let (child, stdin, stdout, stderr) = runtime.block_on(async {
            let mut cmd = Command::new(&cfg.plugin.cmd);
            cmd.args(&cfg.plugin.args)
                .arg(&host_path)
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .stderr(Stdio::piped());
            let mut child = cmd
                .spawn()
                .map_err(|e| PluginError::SpawnFailed { detail: e.to_string() })?;
            let stdin = child.stdin.take().ok_or_else(|| PluginError::IoFailed {
                detail: "stdin unavailable".to_string(),
            })?;
            let stdout = child.stdout.take().ok_or_else(|| PluginError::IoFailed {
                detail: "stdout unavailable".to_string(),
            })?;
            let stderr = child.stderr.take().ok_or_else(|| PluginError::IoFailed {
                detail: "stderr unavailable".to_string(),
            })?;
            Ok((child, stdin, stdout, stderr))
        })?;
        let stdout_rx = spawn_stdout(&runtime, stdout);
        let stderr_buf = spawn_stderr(&runtime, stderr);
        let mut host = Self {
            runtime,
            child,
            stdin,
            stdout_rx,
            stderr_buf,
            stderr_pos: 0,
            timeout,
            snippet_limit,
            severity_override,
        };
        host.init(scripts)?;
        Ok(host)
    }

    pub fn run_stage(
        &mut self,
        stage: &Stage,
        input_path: &Path,
        payload: StagePayload<'_>,
    ) -> Result<Vec<Violation>, PluginError> {
        let path_s = input_path.to_string_lossy().into_owned();
        let stage_name = stage.as_str();
        log_event(Ev::new(Event::PluginInvoke, &path_s).with_stage(stage_name));
        let t0 = Instant::now();
        let payload_value =
            serde_json::to_value(&payload).map_err(|e| PluginError::BadJson { detail: e.to_string() })?;
        let req = HostRequest::RunStage {
            stage: stage_name,
            path: input_path,
            payload: payload_value,
        };
        self.send(&req)?;
        let resp = match self.recv() {
            Ok(r) => r,
            Err(e @ PluginError::Timeout { .. }) => {
                let elapsed = t0.elapsed().as_millis();
                log_event(
                    Ev::new(Event::PluginTimeout, &path_s)
                        .with_stage(stage_name)
                        .with_duration_ms(elapsed),
                );
                return Err(e);
            }
            Err(e) => return Err(e),
        };
        let violations = match resp {
            HostResponse::Violations { violations } => violations,
            HostResponse::Error { detail } => {
                let detail = detail.unwrap_or_else(|| "plugin error".to_string());
                return Err(PluginError::ProtocolError { detail });
            }
            HostResponse::Ready => {
                return Err(PluginError::ProtocolError {
                    detail: "unexpected ready response".to_string(),
                })
            }
        };
        let adjusted = self.apply_overrides(violations);
        self.log_stderr(&path_s, stage_name);
        let elapsed = t0.elapsed().as_millis();
        log_event(
            Ev::new(Event::PluginDone, &path_s)
                .with_stage(stage_name)
                .with_duration_ms(elapsed),
        );
        Ok(adjusted)
    }

    fn init(&mut self, scripts: Vec<String>) -> Result<(), PluginError> {
        let req = HostRequest::Init { scripts: &scripts };
        self.send(&req)?;
        match self.recv()? {
            HostResponse::Ready => Ok(()),
            HostResponse::Error { detail } => Err(PluginError::ProtocolError {
                detail: detail.unwrap_or_else(|| "init failed".to_string()),
            }),
            HostResponse::Violations { .. } => Err(PluginError::ProtocolError {
                detail: "unexpected violations response during init".to_string(),
            }),
        }
    }

    fn send(&mut self, req: &HostRequest<'_>) -> Result<(), PluginError> {
        let data = serde_json::to_vec(req).map_err(|e| PluginError::BadJson { detail: e.to_string() })?;
        let stdin = &mut self.stdin;
        self.runtime
            .block_on(async {
                stdin.write_all(&data).await?;
                stdin.write_all(b"\n").await?;
                stdin.flush().await
            })
            .map_err(|e| PluginError::IoFailed { detail: e.to_string() })
    }

    fn recv(&mut self) -> Result<HostResponse, PluginError> {
        let timeout = self.timeout;
        let stdout_rx = &mut self.stdout_rx;
        let child = &mut self.child;
        self.runtime.block_on(async {
            match time::timeout(timeout, stdout_rx.recv()).await {
                Ok(Some(line)) => {
                    serde_json::from_str(&line).map_err(|e| PluginError::BadJson { detail: e.to_string() })
                }
                Ok(None) => Err(PluginError::ProtocolError {
                    detail: "host closed stdout".to_string(),
                }),
                Err(_) => {
                    let _ = child.start_kill();
                    let _ = child.wait().await;
                    Err(PluginError::Timeout {
                        timeout_ms: timeout.as_millis() as u64,
                    })
                }
            }
        })
    }

    fn apply_overrides(&self, mut violations: Vec<Violation>) -> Vec<Violation> {
        for v in &mut violations {
            if let Some(sev) = self.severity_override.get(&v.rule_id) {
                v.severity = *sev;
            }
        }
        violations
    }

    fn log_stderr(&mut self, path: &str, stage: &str) {
        if self.snippet_limit == 0 {
            return;
        }
        if let Ok(buf) = self.stderr_buf.lock() {
            if buf.len() <= self.stderr_pos {
                return;
            }
            self.stderr_pos = buf.len();
            let slice = if buf.len() > self.snippet_limit {
                &buf[buf.len() - self.snippet_limit..]
            } else {
                &buf[..]
            };
            if !slice.is_empty() {
                let snippet = String::from_utf8_lossy(slice).to_string();
                log_event(
                    Ev::new(Event::PluginStderr, path)
                        .with_stage(stage)
                        .with_stderr_snippet(&snippet),
                );
            }
        }
    }
}

impl Drop for PythonHost {
    fn drop(&mut self) {
        let _ = self.send(&HostRequest::Shutdown);
        let child = &mut self.child;
        let _ = self.runtime.block_on(async {
            let _ = child.start_kill();
            let _ = child.wait().await;
        });
    }
}

fn spawn_stdout(runtime: &Runtime, stdout: ChildStdout) -> UnboundedReceiver<String> {
    let (tx, rx) = tokio::sync::mpsc::unbounded_channel();
    runtime.spawn(async move {
        read_stdout(stdout, tx).await;
    });
    rx
}

fn spawn_stderr(runtime: &Runtime, stderr: ChildStderr) -> Arc<Mutex<Vec<u8>>> {
    let buf = Arc::new(Mutex::new(Vec::new()));
    let dst = buf.clone();
    runtime.spawn(async move {
        read_stderr(stderr, dst).await;
    });
    buf
}

async fn read_stdout(stdout: ChildStdout, tx: UnboundedSender<String>) {
    let mut reader = BufReader::new(stdout);
    let mut line = String::new();
    loop {
        line.clear();
        match reader.read_line(&mut line).await {
            Ok(0) => break,
            Ok(_) => {
                while line.ends_with('\n') || line.ends_with('\r') {
                    line.pop();
                }
                let _ = tx.send(line.clone());
            }
            Err(_) => break,
        }
    }
}

async fn read_stderr(stderr: ChildStderr, dst: Arc<Mutex<Vec<u8>>>) {
    let mut reader = BufReader::new(stderr);
    let mut tmp = [0u8; 4096];
    loop {
        match reader.read(&mut tmp).await {
            Ok(0) => break,
            Ok(n) => {
                if let Ok(mut buf) = dst.lock() {
                    buf.extend_from_slice(&tmp[..n]);
                }
            }
            Err(_) => break,
        }
    }
}

fn build_severity_override(raw: &HashMap<String, String>) -> HashMap<String, Severity> {
    let mut map = HashMap::new();
    for (k, v) in raw {
        if let Some(sev) = parse_severity(v) {
            map.insert(k.clone(), sev);
        }
    }
    map
}

fn parse_severity(s: &str) -> Option<Severity> {
    match s {
        "error" => Some(Severity::Error),
        "warning" => Some(Severity::Warning),
        "info" => Some(Severity::Info),
        _ => None,
    }
}
