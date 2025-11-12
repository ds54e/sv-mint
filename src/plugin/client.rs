use crate::config::Config;
use crate::core::errors::PluginError;
use crate::diag::event::{Ev, Event};
use crate::diag::logging::log_event;
use crate::plugin_scripts::{resolve_script_path, resolve_scripts};
use crate::types::{Severity, Stage, Violation};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::io::{BufRead, BufReader, Read, Write};
use std::path::Path;
use std::process::{Child, ChildStdin, Command, Stdio};
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

pub struct PythonHost {
    child: Child,
    stdin: ChildStdin,
    stdout_rx: mpsc::Receiver<String>,
    stderr_buf: Arc<Mutex<Vec<u8>>>,
    stderr_pos: usize,
    timeout: Duration,
    snippet_limit: usize,
    severity_override: std::collections::HashMap<String, Severity>,
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
        let stdout_rx = spawn_stdout(stdout);
        let stderr_buf = spawn_stderr(stderr);
        let timeout = Duration::from_millis(cfg.defaults.timeout_ms_per_file);
        let snippet_limit = cfg.logging.stderr_snippet_bytes;
        let severity_override = build_severity_override(&cfg.ruleset.severity_override);
        let mut host = Self {
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
        payload: Value,
    ) -> Result<Vec<Violation>, PluginError> {
        let path_s = input_path.to_string_lossy().into_owned();
        let stage_name = stage.as_str();
        log_event(Ev::new(Event::PluginInvoke, &path_s).with_stage(stage_name));
        let t0 = Instant::now();
        let req = HostRequest::RunStage {
            stage: stage_name,
            path: input_path,
            payload,
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
        let text = serde_json::to_vec(req).map_err(|e| PluginError::BadJson { detail: e.to_string() })?;
        self.stdin
            .write_all(&text)
            .and_then(|_| self.stdin.write_all(b"\n"))
            .and_then(|_| self.stdin.flush())
            .map_err(|e| PluginError::IoFailed { detail: e.to_string() })
    }

    fn recv(&mut self) -> Result<HostResponse, PluginError> {
        match self.stdout_rx.recv_timeout(self.timeout) {
            Ok(line) => serde_json::from_str(&line).map_err(|e| PluginError::BadJson { detail: e.to_string() }),
            Err(mpsc::RecvTimeoutError::Timeout) => {
                let _ = self.child.kill();
                Err(PluginError::Timeout {
                    timeout_ms: self.timeout.as_millis() as u64,
                })
            }
            Err(_) => Err(PluginError::ProtocolError {
                detail: "host closed stdout".to_string(),
            }),
        }
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
        let req = HostRequest::Shutdown;
        let _ = self.send(&req);
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}

fn spawn_stdout(stdout: impl Read + Send + 'static) -> mpsc::Receiver<String> {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let mut reader = BufReader::new(stdout);
        loop {
            let mut line = String::new();
            match reader.read_line(&mut line) {
                Ok(0) => break,
                Ok(_) => {
                    if line.ends_with('\n') {
                        line.pop();
                        if line.ends_with('\r') {
                            line.pop();
                        }
                    }
                    let _ = tx.send(line);
                }
                Err(_) => break,
            }
        }
    });
    rx
}

fn spawn_stderr(stderr: impl Read + Send + 'static) -> Arc<Mutex<Vec<u8>>> {
    let buf = Arc::new(Mutex::new(Vec::new()));
    let buf_clone = buf.clone();
    thread::spawn(move || {
        let mut reader = BufReader::new(stderr);
        let mut tmp = [0u8; 4096];
        loop {
            match reader.read(&mut tmp) {
                Ok(0) => break,
                Ok(n) => {
                    if let Ok(mut dst) = buf_clone.lock() {
                        dst.extend_from_slice(&tmp[..n]);
                    }
                }
                Err(_) => break,
            }
        }
    });
    buf
}

fn build_severity_override(
    raw: &std::collections::HashMap<String, String>,
) -> std::collections::HashMap<String, Severity> {
    let mut map = std::collections::HashMap::new();
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
