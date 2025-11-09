use crate::config::Config;
use crate::types::Violation;
use anyhow::{anyhow, bail, ensure, Context, Result};
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

pub fn run_plugin_once(
    cfg_dir: &Path,
    cfg: &Config,
    stage: &str,
    input_path: &Path,
    payload: Value,
) -> Result<Vec<Violation>> {
    let (cmd, argv) = resolve_plugin_cmd_and_args(cfg_dir, &cfg.plugin);
    let req = PluginReq {
        ty: "CheckFileStage",
        stage,
        path: &input_path.to_string_lossy(),
        payload,
    };
    let raw = call_plugin(
        &cmd,
        &argv,
        &serde_json::to_string(&req).expect("serialize request"),
        Duration::from_millis(cfg.defaults.timeout_ms_per_file),
    )?;
    if raw.trim().is_empty() {
        bail!("invalid response json");
    }
    let resp: PluginResp = serde_json::from_str(&raw).map_err(|e| anyhow!("invalid response json: {}", e))?;
    ensure!(resp.ty == "ViolationsStage", "protocol error");
    ensure!(resp.stage == stage, "protocol error");
    Ok(resp.violations)
}

fn resolve_plugin_cmd_and_args(cfg_dir: &Path, plugin: &crate::config::Plugin) -> (String, Vec<String>) {
    let mut argv = Vec::new();
    for a in &plugin.args {
        if a.ends_with(".py") || a.contains(std::path::is_separator) {
            argv.push(cfg_dir.join(a).to_string_lossy().to_string());
        } else {
            argv.push(a.clone());
        }
    }
    (plugin.cmd.clone(), argv)
}

fn call_plugin(cmd: &str, args: &[String], request_json: &str, timeout: Duration) -> Result<String> {
    let mut child = Command::new(cmd)
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .with_context(|| format!("failed to spawn plugin: {} {:?}", cmd, args))?;

    if let Some(mut stdin) = child.stdin.take() {
        stdin
            .write_all(request_json.as_bytes())
            .context("failed to write request to plugin stdin")?;
        let _ = stdin.flush();
    } else {
        return Err(anyhow!("plugin stdin not available"));
    }

    let mut stdout = child
        .stdout
        .take()
        .ok_or_else(|| anyhow!("plugin stdout not available"))?;
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
                        let _ = tx_out.send(Err(anyhow!("stdout too large")));
                        return;
                    }
                    buf.extend_from_slice(&tmp[..n]);
                }
                Err(e) => {
                    let _ = tx_out.send(Err(anyhow!("failed to read stdout: {}", e)));
                    return;
                }
            }
        }
        match String::from_utf8(buf) {
            Ok(s) => {
                let _ = tx_out.send(Ok(s));
            }
            Err(e) => {
                let _ = tx_out.send(Err(anyhow!("stdout is not valid UTF-8: {}", e)));
            }
        }
    });

    let mut stderr = child
        .stderr
        .take()
        .ok_or_else(|| anyhow!("plugin stderr not available"))?;
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
                        let _ = tx_err.send(Err(anyhow!("stderr too large")));
                        return;
                    }
                    buf.extend_from_slice(&tmp[..n]);
                }
                Err(e) => {
                    let _ = tx_err.send(Err(anyhow!("failed to read stderr: {}", e)));
                    return;
                }
            }
        }
        let s = String::from_utf8_lossy(&buf).to_string();
        let _ = tx_err.send(Ok(s));
    });

    let start = Instant::now();
    let out = match rx_out.recv_timeout(timeout) {
        Ok(res) => res,
        Err(mpsc::RecvTimeoutError::Timeout) => {
            let _ = child.kill();
            let _ = child.wait();
            let err_snapshot = rx_err.try_recv().ok().and_then(|r| r.ok()).unwrap_or_default();
            return Err(anyhow!("timeout").context(truncate_for_context(err_snapshot)));
        }
        Err(e) => {
            let _ = child.kill();
            let _ = child.wait();
            return Err(anyhow!("failed to receive stdout: {}", e));
        }
    }?;

    let remaining = timeout.checked_sub(start.elapsed()).unwrap_or(Duration::from_millis(0));
    wait_with_timeout(&mut child, remaining)?;
    let err_text = match rx_err.recv_timeout(Duration::from_millis(50)) {
        Ok(Ok(s)) => s,
        Ok(Err(e)) => format!("stderr read error: {e}"),
        Err(_) => String::new(),
    };
    if let Ok(None) = child.try_wait() {
        let _ = child.kill();
        let _ = child.wait();
    }
    if !err_text.trim().is_empty() {
        let shown = truncate_preview(&err_text, 8 * 1024);
        eprintln!("[plugin stderr] {}", shown.trim_end());
    }

    Ok(out)
}

fn wait_with_timeout(child: &mut Child, dur: Duration) -> Result<()> {
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
            Err(e) => return Err(anyhow!(e)),
        }
    }
}

fn truncate_for_context(s: String) -> anyhow::Error {
    let shown = truncate_preview(&s, 2048);
    anyhow!(shown)
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
