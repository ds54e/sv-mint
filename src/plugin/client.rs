use crate::config::Config;
use crate::core::errors::PluginError;
use crate::diag::event::{Ev, Event};
use crate::diag::logging::log_event;
use crate::plugin_scripts::{resolve_script_path, resolve_scripts};
use crate::types::{Severity, Stage, Violation};
use serde_json::{json, Value};
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
        let req = json!({
            "kind": "run_stage",
            "stage": stage_name,
            "path": input_path,
            "payload": payload,
        });
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
        if resp.kind == "error" {
            let detail = resp
                .value
                .get("detail")
                .and_then(|v| v.as_str())
                .unwrap_or("plugin error")
                .to_string();
            return Err(PluginError::ProtocolError { detail });
        }
        if resp.kind != "violations" {
            return Err(PluginError::ProtocolError {
                detail: "unexpected response".to_string(),
            });
        }
        let violations: Vec<Violation> = serde_json::from_value(
            resp.value
                .get("violations")
                .cloned()
                .unwrap_or_else(|| Value::Array(Vec::new())),
        )
        .map_err(|e| PluginError::BadJson { detail: e.to_string() })?;
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
        let req = json!({
            "kind": "init",
            "scripts": scripts,
        });
        self.send(&req)?;
        let resp = self.recv()?;
        if resp.kind != "ready" {
            return Err(PluginError::ProtocolError {
                detail: "init failed".to_string(),
            });
        }
        Ok(())
    }

    fn send(&mut self, value: &Value) -> Result<(), PluginError> {
        let text = serde_json::to_vec(value).map_err(|e| PluginError::BadJson { detail: e.to_string() })?;
        self.stdin
            .write_all(&text)
            .and_then(|_| self.stdin.write_all(b"\n"))
            .and_then(|_| self.stdin.flush())
            .map_err(|e| PluginError::IoFailed { detail: e.to_string() })
    }

    fn recv(&mut self) -> Result<HostMessage, PluginError> {
        match self.stdout_rx.recv_timeout(self.timeout) {
            Ok(line) => {
                let v: Value =
                    serde_json::from_str(&line).map_err(|e| PluginError::BadJson { detail: e.to_string() })?;
                let kind = v
                    .get("type")
                    .or_else(|| v.get("kind"))
                    .and_then(|s| s.as_str())
                    .unwrap_or("")
                    .to_string();
                Ok(HostMessage { kind, value: v })
            }
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
        let _ = self.send(&json!({ "kind": "shutdown" }));
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}

struct HostMessage {
    kind: String,
    value: Value,
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

trait EvExt<'a> {
    fn with_stage(self, s: &'a str) -> Self;
    fn with_duration_ms(self, ms: u128) -> Self;
    fn with_stderr_snippet(self, s: &'a str) -> Self;
}

impl<'a> EvExt<'a> for Ev<'a> {
    fn with_stage(mut self, s: &'a str) -> Self {
        self.stage = Some(s);
        self
    }

    fn with_duration_ms(mut self, ms: u128) -> Self {
        self.duration_ms = Some(ms);
        self
    }

    fn with_stderr_snippet(mut self, s: &'a str) -> Self {
        self.stderr_snippet = Some(s);
        self
    }
}
