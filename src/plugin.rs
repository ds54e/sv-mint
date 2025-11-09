use crate::config::Config;
use crate::output::Violation;
use anyhow::{anyhow, bail, ensure, Result};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::io::{BufRead, BufReader, Write};
use std::path::Path;
use std::process::{Command, Stdio};
use std::sync::mpsc;
use std::time::Duration;

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

    let mut child = Command::new(cmd)
        .args(argv)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .map_err(|_| anyhow!("failed to spawn plugin"))?;

    {
        let mut stdin = child.stdin.take().ok_or_else(|| anyhow!("protocol error"))?;
        let req = PluginReq {
            ty: "CheckFileStage",
            stage,
            path: &input_path.to_string_lossy(),
            payload,
        };
        serde_json::to_writer(&mut stdin, &req)?;
        stdin.write_all(b"\n")?;
    }

    let mut stdout = BufReader::new(child.stdout.take().ok_or_else(|| anyhow!("protocol error"))?);
    let (tx, rx) = mpsc::channel::<String>();
    std::thread::spawn(move || {
        let mut line = String::new();
        let _ = stdout.read_line(&mut line);
        let _ = tx.send(line);
    });

    let to = Duration::from_millis(cfg.defaults.timeout_ms_per_file);
    let line = rx.recv_timeout(to).map_err(|_| anyhow!("timeout"))?;
    if line.trim().is_empty() {
        bail!("invalid response json");
    }
    let resp: PluginResp = serde_json::from_str(&line)?;
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
