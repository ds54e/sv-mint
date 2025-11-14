use crate::config::Config;
use crate::core::errors::PluginError;
use crate::core::payload::StagePayload;
use crate::diag::event::{Ev, Event};
use crate::diag::logging::log_event;
use crate::plugin_scripts::{collect_script_specs, resolve_script_path, ScriptSpec};
use crate::types::{Severity, Stage, Violation};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::{BTreeMap, HashMap};
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
    rule_enabled: HashMap<String, bool>,
}

#[derive(Serialize)]
struct ScriptInit<'a> {
    path: &'a str,
    stages: &'a [String],
    stage_rules: &'a BTreeMap<String, Vec<String>>,
}

#[derive(Serialize)]
pub struct RuleDispatch<'a> {
    #[serde(default)]
    pub enabled: &'a [String],
    #[serde(default)]
    pub disabled: &'a [String],
}

#[derive(Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
enum HostRequest<'a> {
    Init {
        scripts: &'a [ScriptInit<'a>],
    },
    RunStage {
        stage: &'a str,
        path: &'a Path,
        payload: Value,
        rules: RuleDispatch<'a>,
    },
    Shutdown,
}

#[derive(Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
enum HostResponse {
    Ready,
    Violations {
        violations: Vec<Violation>,
    },
    Error {
        detail: Option<String>,
        script: Option<String>,
    },
}

pub struct StageRunResult {
    pub violations: Vec<Violation>,
    pub response_bytes: usize,
}

impl PythonHost {
    pub fn start(cfg: &Config) -> Result<Self, PluginError> {
        let script_specs = collect_script_specs(cfg);
        let host_path = resolve_script_path("plugins/lib/rule_host.py");
        let cmd_preview = format_plugin_command(&cfg.plugin.cmd, &cfg.plugin.args, &host_path);
        let runtime = Runtime::new().map_err(|e| PluginError::SpawnFailed { detail: e.to_string() })?;
        let timeout = Duration::from_millis(cfg.defaults.timeout_ms_per_file);
        let snippet_limit = cfg.logging.stderr_snippet_bytes;
        let severity_override = build_severity_override(&cfg.rule);
        let rule_enabled = build_rule_enabled(&cfg.rule);
        let (child, stdin, stdout, stderr) = runtime.block_on(async {
            let mut cmd = Command::new(&cfg.plugin.cmd);
            cmd.args(&cfg.plugin.args)
                .arg(&host_path)
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .stderr(Stdio::piped());
            let mut child = cmd.spawn().map_err(|e| PluginError::SpawnFailed {
                detail: format!("{cmd_preview}: {e}"),
            })?;
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
            rule_enabled,
        };
        host.init(&script_specs)?;
        Ok(host)
    }

    pub fn run_stage(
        &mut self,
        stage: &Stage,
        input_path: &Path,
        payload: StagePayload<'_>,
        rules: RuleDispatch<'_>,
    ) -> Result<StageRunResult, PluginError> {
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
            rules,
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
                self.log_stderr(&path_s, stage_name);
                return Err(e);
            }
            Err(e @ PluginError::ExitCode { code }) => {
                let elapsed = t0.elapsed().as_millis();
                log_event(
                    Ev::new(Event::PluginExitNonzero, &path_s)
                        .with_stage(stage_name)
                        .with_duration_ms(elapsed)
                        .with_exit_code(code),
                );
                self.log_stderr(&path_s, stage_name);
                return Err(e);
            }
            Err(e) => {
                let elapsed = t0.elapsed().as_millis();
                let detail = e.to_string();
                let mut ev = Ev::new(Event::PluginError, &path_s)
                    .with_stage(stage_name)
                    .with_duration_ms(elapsed);
                ev = ev.with_message(&detail);
                log_event(ev);
                self.log_stderr(&path_s, stage_name);
                return Err(e);
            }
        };
        let (resp, response_bytes) = resp;
        let violations = match resp {
            HostResponse::Violations { violations } => violations,
            HostResponse::Error { detail, script } => {
                let mut detail = detail.unwrap_or_else(|| "plugin error".to_string());
                if let Some(script_path) = script.as_deref() {
                    detail.push_str(&format!(" (script {script_path})"));
                }
                let elapsed = t0.elapsed().as_millis();
                let mut ev = Ev::new(Event::PluginError, &path_s)
                    .with_stage(stage_name)
                    .with_duration_ms(elapsed);
                ev = ev.with_message(&detail);
                log_event(ev);
                self.log_stderr(&path_s, stage_name);
                return Err(PluginError::ProtocolError { detail });
            }
            HostResponse::Ready => {
                let detail = "unexpected ready response".to_string();
                let elapsed = t0.elapsed().as_millis();
                let mut ev = Ev::new(Event::PluginError, &path_s)
                    .with_stage(stage_name)
                    .with_duration_ms(elapsed);
                ev = ev.with_message(&detail);
                log_event(ev);
                self.log_stderr(&path_s, stage_name);
                return Err(PluginError::ProtocolError { detail });
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
        Ok(StageRunResult {
            violations: adjusted,
            response_bytes,
        })
    }

    fn init(&mut self, scripts: &[ScriptSpec]) -> Result<(), PluginError> {
        let payload: Vec<_> = scripts
            .iter()
            .map(|spec| ScriptInit {
                path: spec.path.as_str(),
                stages: spec.stages.as_slice(),
                stage_rules: &spec.stage_rules,
            })
            .collect();
        let req = HostRequest::Init { scripts: &payload };
        self.send(&req)?;
        match self.recv()? {
            (HostResponse::Ready, _) => Ok(()),
            (HostResponse::Error { detail, .. }, _) => Err(PluginError::ProtocolError {
                detail: detail.unwrap_or_else(|| "init failed".to_string()),
            }),
            (HostResponse::Violations { .. }, _) => Err(PluginError::ProtocolError {
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

    fn recv(&mut self) -> Result<(HostResponse, usize), PluginError> {
        let timeout = self.timeout;
        let stdout_rx = &mut self.stdout_rx;
        let child = &mut self.child;
        self.runtime.block_on(async {
            match time::timeout(timeout, stdout_rx.recv()).await {
                Ok(Some(line)) => {
                    let len = line.len();
                    serde_json::from_str(&line)
                        .map(|resp| (resp, len))
                        .map_err(|e| PluginError::BadJson { detail: e.to_string() })
                }
                Ok(None) => {
                    let status = child
                        .wait()
                        .await
                        .map_err(|e| PluginError::IoFailed { detail: e.to_string() })?;
                    let code = status.code().unwrap_or(-1);
                    Err(PluginError::ExitCode { code })
                }
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

    fn apply_overrides(&self, violations: Vec<Violation>) -> Vec<Violation> {
        let mut out = Vec::with_capacity(violations.len());
        for mut v in violations {
            if matches!(self.rule_enabled.get(&v.rule_id), Some(false)) {
                continue;
            }
            if let Some(sev) = self.severity_override.get(&v.rule_id) {
                v.severity = *sev;
            }
            out.push(v);
        }
        out
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
        self.runtime.block_on(async {
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

fn build_severity_override(rules: &[crate::config::RuleConfig]) -> HashMap<String, Severity> {
    let mut map = HashMap::new();
    for rule in rules {
        if let Some(sev_name) = &rule.severity {
            if let Some(sev) = parse_severity(sev_name) {
                map.insert(rule.id.clone(), sev);
            }
        }
    }
    map
}

fn build_rule_enabled(rules: &[crate::config::RuleConfig]) -> HashMap<String, bool> {
    let mut map = HashMap::new();
    for rule in rules {
        map.insert(rule.id.clone(), rule.enabled);
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

fn format_plugin_command(cmd: &str, args: &[String], host_path: &str) -> String {
    let mut parts = Vec::with_capacity(args.len() + 2);
    parts.push(cmd.to_string());
    parts.extend(args.iter().cloned());
    parts.push(host_path.to_string());
    let mut out = String::new();
    for (i, part) in parts.iter().enumerate() {
        if i > 0 {
            out.push(' ');
        }
        if part.chars().any(|c| c.is_whitespace()) {
            out.push_str(&format!("{part:?}"));
        } else {
            out.push_str(part);
        }
    }
    out
}
