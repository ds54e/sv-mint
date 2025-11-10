use crate::config::Config;
use crate::errors::PluginError;
use crate::types::Violation;
use log::{info, warn};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::io::{Read, Write};
use std::path::Path;
use std::process::{Child, Command, Stdio};
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};

const MAX_STDOUT_BYTES: usize = 16 * 1024 * 1024;
const MAX_STDERR_BYTES: usize = 4 * 1024 * 1024;

#[derive(Serialize)]
struct PluginReq<'a> {
    #[serde(rename = "type")]
    ty: &'static str,
    stage: &'a str,
    path: &'a str,
    payload: Value,
}

#[derive(Deserialize)]
struct PluginResp {
    #[serde(rename = "type")]
    ty: String,
    stage: String,
    violations: Vec<Violation>,
}

#[derive(Clone)]
struct CallOpts<'a> {
    timeout: Duration,
    stderr_snippet_max: usize,
    show_events: bool,
    stage: &'a str,
    input_path: &'a Path,
}

pub fn run_plugin_once(
    cfg_dir: &Path,
    cfg: &Config,
    stage: &str,
    input_path: &Path,
    payload: Value,
) -> Result<Vec<Violation>, PluginError> {
    let (cmd, argv) = resolve_plugin_cmd_and_args(cfg_dir, &cfg.plugin);
    let req = PluginReq {
        ty: "CheckFileStage",
        stage,
        path: &input_path.to_string_lossy(),
        payload,
    };
    if cfg.logging.show_plugin_events {
        info!(
            "event=plugin_invoke stage={} path={} cmd={}",
            stage,
            input_path.display(),
            cmd
        );
    }
    let t0 = Instant::now();
    let raw = match call_plugin(
        &cmd,
        &argv,
        &serde_json::to_string(&req).map_err(|e| PluginError::BadJson { detail: e.to_string() })?,
        CallOpts {
            timeout: Duration::from_millis(cfg.defaults.timeout_ms_per_file),
            stderr_snippet_max: cfg.logging.stderr_snippet_bytes,
            show_events: cfg.logging.show_plugin_events,
            stage,
            input_path,
        },
    ) {
        Ok(s) => s,
        Err(e) => {
            if cfg.logging.show_plugin_events {
                warn!(
                    "event=plugin_error stage={} path={} msg={}",
                    stage,
                    input_path.display(),
                    e
                );
            }
            return Err(e);
        }
    };
    if cfg.logging.show_plugin_events {
        info!(
            "event=plugin_done stage={} path={} elapsed_ms={} resp_bytes={}",
            stage,
            input_path.display(),
            t0.elapsed().as_millis(),
            raw.len()
        );
    }
    if raw.trim().is_empty() {
        return Err(PluginError::BadJson {
            detail: "empty response".to_string(),
        });
    }
    let resp: PluginResp = serde_json::from_str(&raw).map_err(|e| PluginError::BadJson { detail: e.to_string() })?;
    if resp.ty != "ViolationsStage" {
        return Err(PluginError::ProtocolError {
            detail: format!("type={} expected=ViolationsStage", resp.ty),
        });
    }
    if resp.stage != stage {
        return Err(PluginError::ProtocolError {
            detail: format!("stage={} expected={}", resp.stage, stage),
        });
    }
    Ok(resp.violations)
}

fn resolve_plugin_cmd_and_args(cfg_dir: &Path, plugin: &crate::config::Plugin) -> (String, Vec<String>) {
    let mut argv = Vec::new();
    for a in &plugin.args {
        if a.ends_with(".py") || a.chars().any(std::path::is_separator) {
            argv.push(cfg_dir.join(a).to_string_lossy().to_string());
        } else {
            argv.push(a.clone());
        }
    }
    (plugin.cmd.clone(), argv)
}

fn call_plugin(cmd: &str, args: &[String], request_json: &str, opts: CallOpts) -> Result<String, PluginError> {
    let mut child = Command::new(cmd)
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| PluginError::SpawnFailed {
            detail: format!("{} {:?}", cmd, args),
            source: Some(e),
        })?;

    if let Some(mut stdin) = child.stdin.take() {
        stdin
            .write_all(request_json.as_bytes())
            .map_err(|e| PluginError::IoFailed {
                detail: "write stdin".to_string(),
                source: Some(e),
            })?;
        let _ = stdin.flush();
    } else {
        return Err(PluginError::IoFailed {
            detail: "stdin not available".to_string(),
            source: None,
        });
    }

    let mut stdout = child.stdout.take().ok_or_else(|| PluginError::IoFailed {
        detail: "stdout not available".to_string(),
        source: None,
    })?;
    let (tx_out, rx_out) = mpsc::channel();
    thread::spawn(move || {
        let mut buf = Vec::with_capacity(64 * 1024);
        let mut tmp = [0u8; 64 * 1024];
        let mut total: usize = 0;
        loop {
            match stdout.read(&mut tmp) {
                Ok(0) => break,
                Ok(n) => {
                    total += n;
                    if total > MAX_STDOUT_BYTES {
                        let _ = tx_out.send(Err(PluginError::StdoutTooLarge));
                        return;
                    }
                    buf.extend_from_slice(&tmp[..n]);
                }
                Err(e) => {
                    let _ = tx_out.send(Err(PluginError::IoFailed {
                        detail: format!("read stdout: {}", e),
                        source: Some(e),
                    }));
                    return;
                }
            }
        }
        match String::from_utf8(buf) {
            Ok(s) => {
                let _ = tx_out.send(Ok(s));
            }
            Err(_) => {
                let _ = tx_out.send(Err(PluginError::BadUtf8));
            }
        }
    });

    let mut stderr = child.stderr.take().ok_or_else(|| PluginError::IoFailed {
        detail: "stderr not available".to_string(),
        source: None,
    })?;
    let (tx_err, rx_err) = mpsc::channel();
    thread::spawn(move || {
        let mut buf = Vec::with_capacity(16 * 1024);
        let mut tmp = [0u8; 16 * 1024];
        let mut total: usize = 0;
        loop {
            match stderr.read(&mut tmp) {
                Ok(0) => break,
                Ok(n) => {
                    total += n;
                    if total > MAX_STDERR_BYTES {
                        let _ = tx_err.send(Err(PluginError::StderrTooLarge));
                        return;
                    }
                    buf.extend_from_slice(&tmp[..n]);
                }
                Err(e) => {
                    let _ = tx_err.send(Err(PluginError::IoFailed {
                        detail: format!("read stderr: {}", e),
                        source: Some(e),
                    }));
                    return;
                }
            }
        }
        let s = String::from_utf8_lossy(&buf).to_string();
        let _ = tx_err.send(Ok(s));
    });

    let start = Instant::now();
    let out = match rx_out.recv_timeout(opts.timeout) {
        Ok(res) => res,
        Err(mpsc::RecvTimeoutError::Timeout) => {
            let _ = child.kill();
            let _ = child.wait();
            if opts.show_events {
                if let Ok(Ok(sn)) = rx_err.try_recv() {
                    let shown = truncate_preview(&sn, opts.stderr_snippet_max);
                    warn!(
                        "event=plugin_timeout stage={} path={} timeout_ms={} stderr={}",
                        opts.stage,
                        opts.input_path.display(),
                        opts.timeout.as_millis(),
                        shown
                    );
                } else {
                    warn!(
                        "event=plugin_timeout stage={} path={} timeout_ms={}",
                        opts.stage,
                        opts.input_path.display(),
                        opts.timeout.as_millis()
                    );
                }
            }
            return Err(PluginError::Timeout {
                timeout_ms: opts.timeout.as_millis() as u64,
            });
        }
        Err(e) => {
            let _ = child.kill();
            let _ = child.wait();
            return Err(PluginError::IoFailed {
                detail: format!("receive stdout: {}", e),
                source: None,
            });
        }
    }?;

    let remaining = opts
        .timeout
        .checked_sub(start.elapsed())
        .unwrap_or(Duration::from_millis(0));
    let _ = wait_with_timeout(&mut child, remaining);
    let err_text = match rx_err.recv_timeout(Duration::from_millis(50)) {
        Ok(Ok(s)) => s,
        Ok(Err(_)) => String::new(),
        Err(_) => String::new(),
    };

    let status = child.wait().ok();
    if let Some(st) = status {
        if !st.success() {
            let code = st.code().unwrap_or(-1);
            if opts.show_events && !err_text.trim().is_empty() {
                let shown = truncate_preview(&err_text, opts.stderr_snippet_max);
                warn!(
                    "event=plugin_exit_nonzero stage={} path={} code={} stderr={}",
                    opts.stage,
                    opts.input_path.display(),
                    code,
                    shown
                );
            }
            return Err(PluginError::ExitCode { code });
        }
    } else {
        let _ = child.kill();
        let _ = child.wait();
    }

    if opts.show_events && !err_text.trim().is_empty() {
        let shown = truncate_preview(&err_text, opts.stderr_snippet_max);
        warn!(
            "event=plugin_stderr stage={} path={} stderr={}",
            opts.stage,
            opts.input_path.display(),
            shown.trim_end()
        );
    }

    Ok(out)
}

fn wait_with_timeout(child: &mut Child, dur: Duration) -> Result<(), PluginError> {
    if dur.is_zero() {
        return Ok(());
    }
    let deadline = Instant::now() + dur;
    loop {
        match child.try_wait() {
            Ok(Some(_)) => return Ok(()),
            Ok(None) => {
                if Instant::now() >= deadline {
                    return Ok(());
                }
                thread::sleep(Duration::from_millis(10));
            }
            Err(e) => {
                return Err(PluginError::IoFailed {
                    detail: "try_wait".to_string(),
                    source: Some(e),
                })
            }
        }
    }
}

fn truncate_preview(s: &str, max: usize) -> String {
    if s.len() <= max {
        s.to_owned()
    } else {
        let mut end = max.min(s.len());
        while end > 0 && !s.is_char_boundary(end) {
            end -= 1;
        }
        if end == 0 {
            String::new()
        } else {
            let mut t = s[..end].to_owned();
            t.push_str(" ...");
            t
        }
    }
}
