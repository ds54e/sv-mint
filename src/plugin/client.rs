use crate::config::Config;
use crate::core::errors::PluginError;
use crate::core::types::Violation;
use crate::diag::event::{Ev, Event};
use crate::diag::logging::log_event;
use crate::plugin::limits::TransportLimits;
use crate::plugin::protocol::{CheckFileStageRequest, ViolationsStageResponse};
use serde_json::Value;
use std::io::{Read, Write};
use std::path::Path;
use std::process::{Command, Stdio};
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};

pub fn run_plugin_once(
    cfg: &Config,
    stage: &str,
    input_path: &Path,
    payload: Value,
) -> Result<Vec<Violation>, PluginError> {
    let path_s = input_path.to_string_lossy().into_owned();
    let req = CheckFileStageRequest::new(stage, &path_s, payload);
    log_event(Ev::new(Event::PluginInvoke, &path_s).with_stage(stage));
    let t0 = Instant::now();
    let limits = TransportLimits::default();

    let mut child = Command::new(&cfg.plugin.cmd)
        .args(&cfg.plugin.args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| PluginError::SpawnFailed { detail: e.to_string() })?;

    let mut sin = child.stdin.take().ok_or_else(|| PluginError::IoFailed {
        detail: "stdin unavailable".to_string(),
    })?;
    let req_text = serde_json::to_string(&req).map_err(|e| PluginError::BadJson { detail: e.to_string() })?;
    sin.write_all(req_text.as_bytes())
        .map_err(|e| PluginError::IoFailed { detail: e.to_string() })?;
    drop(sin);

    let mut so = child.stdout.take().ok_or_else(|| PluginError::IoFailed {
        detail: "stdout unavailable".to_string(),
    })?;
    let mut se = child.stderr.take().ok_or_else(|| PluginError::IoFailed {
        detail: "stderr unavailable".to_string(),
    })?;

    let (tx_out, rx_out) = mpsc::channel();
    let (tx_err, rx_err) = mpsc::channel();

    thread::spawn(move || {
        let r = read_limited(&mut so, limits.stdout_max, true);
        let _ = tx_out.send(r);
    });
    thread::spawn(move || {
        let r = read_limited(&mut se, limits.stderr_max, false);
        let _ = tx_err.send(r);
    });

    let timeout = Duration::from_millis(cfg.defaults.timeout_ms_per_file);
    let status = wait_with_timeout(&mut child, timeout);

    let out_res = rx_out.recv().unwrap_or_else(|_| Ok(Vec::new()));
    let err_res = rx_err.recv().unwrap_or_else(|_| Ok(Vec::new()));

    match status {
        WaitOutcome::Timeout => {
            let _ = child.kill();
            let _ = child.wait();
            let elapsed = t0.elapsed().as_millis();
            log_event(
                Ev::new(Event::PluginTimeout, &path_s)
                    .with_stage(stage)
                    .with_duration_ms(elapsed),
            );
            Err(PluginError::Timeout {
                timeout_ms: cfg.defaults.timeout_ms_per_file,
            })
        }
        WaitOutcome::IoErr(e) => Err(PluginError::IoFailed { detail: e }),
        WaitOutcome::Exited(code) => {
            let stderr_bytes = err_res?;
            if !stderr_bytes.is_empty() && cfg.logging.stderr_snippet_bytes > 0 {
                let n = cfg.logging.stderr_snippet_bytes.min(stderr_bytes.len());
                let snippet = String::from_utf8_lossy(&stderr_bytes[..n]).to_string();
                log_event(
                    Ev::new(Event::PluginStderr, &path_s)
                        .with_stage(stage)
                        .with_stderr_snippet(&snippet),
                );
            }
            if code != 0 {
                let elapsed = t0.elapsed().as_millis();
                log_event(
                    Ev::new(Event::PluginExitNonzero, &path_s)
                        .with_stage(stage)
                        .with_duration_ms(elapsed)
                        .with_exit_code(code),
                );
                return Err(PluginError::ExitCode { code });
            }
            let stdout_bytes = out_res?;
            let stdout_text =
                String::from_utf8(stdout_bytes).map_err(|e| PluginError::BadUtf8 { detail: e.to_string() })?;
            let resp: ViolationsStageResponse =
                serde_json::from_str(&stdout_text).map_err(|e| PluginError::BadJson { detail: e.to_string() })?;
            if resp.ty.as_str() != "ViolationsStage" {
                return Err(PluginError::ProtocolError {
                    detail: "unexpected type".to_string(),
                });
            }
            if resp.stage.as_str() != stage {
                return Err(PluginError::ProtocolError {
                    detail: "stage mismatch".to_string(),
                });
            }
            let elapsed = t0.elapsed().as_millis();
            log_event(
                Ev::new(Event::PluginDone, &path_s)
                    .with_stage(stage)
                    .with_duration_ms(elapsed),
            );
            Ok(resp.violations)
        }
    }
}

fn read_limited<R: Read>(mut r: R, cap: usize, is_stdout: bool) -> Result<Vec<u8>, PluginError> {
    let mut buf = Vec::with_capacity(8192);
    let mut tmp = [0u8; 8192];
    loop {
        match r.read(&mut tmp) {
            Ok(0) => break,
            Ok(n) => {
                if buf.len() + n > cap {
                    return if is_stdout {
                        Err(PluginError::StdoutTooLarge)
                    } else {
                        Err(PluginError::StderrTooLarge)
                    };
                }
                buf.extend_from_slice(&tmp[..n]);
            }
            Err(e) => return Err(PluginError::IoFailed { detail: e.to_string() }),
        }
    }
    Ok(buf)
}

enum WaitOutcome {
    Timeout,
    IoErr(String),
    Exited(i32),
}

fn wait_with_timeout(child: &mut std::process::Child, timeout: Duration) -> WaitOutcome {
    let t0 = Instant::now();
    loop {
        match child.try_wait() {
            Ok(Some(status)) => {
                let code = status.code().unwrap_or(-1);
                return WaitOutcome::Exited(code);
            }
            Ok(None) => {
                if t0.elapsed() >= timeout {
                    return WaitOutcome::Timeout;
                }
                std::thread::sleep(Duration::from_millis(10));
            }
            Err(e) => return WaitOutcome::IoErr(e.to_string()),
        }
    }
}

trait EvExt<'a> {
    fn with_stage(self, s: &'a str) -> Self;
    fn with_duration_ms(self, ms: u128) -> Self;
    fn with_exit_code(self, code: i32) -> Self;
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
    fn with_exit_code(mut self, code: i32) -> Self {
        self.exit_code = Some(code);
        self
    }
    fn with_stderr_snippet(mut self, s: &'a str) -> Self {
        self.stderr_snippet = Some(s);
        self
    }
}
