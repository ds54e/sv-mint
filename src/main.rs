use anyhow::{bail, ensure, Context, Result};
use clap::Parser;
use serde::{Deserialize, Serialize};
use serde_json::to_writer;
use std::fs;
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode, Stdio};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

const MAX_INPUT_BYTES: usize = 8 * 1024 * 1024;
const TIMEOUT_MIN_MS: u64 = 100;
const TIMEOUT_MAX_MS: u64 = 60_000;

const E_INVALID_TOML: &str = "invalid toml";
const E_PROTOCOL: &str = "protocol error";
const E_TIMEOUT: &str = "timeout";
const E_SIZE_OVER: &str = "size over";
const E_INVALID_RESP_JSON: &str = "invalid response json";
const E_CONFIG_NOT_FOUND: &str = "config not found";
const E_CONFIG_NOT_FOUND_FLAG: &str = "config not found (from --config)";

#[derive(Parser, Debug)]
#[command(name = "sv-mint", version, about = "SystemVerilog linter (minimal, Windows-only)", long_about = None)]
struct Args {
    #[arg(long = "config")]
    config: Option<PathBuf>,
    #[arg()]
    input: PathBuf,
}

#[derive(Deserialize)]
struct Config {
    #[serde(default)]
    defaults: Defaults,
    plugin: Plugin,
}

#[derive(Deserialize)]
struct Defaults {
    #[serde(default = "Defaults::default_timeout")]
    timeout_ms_per_file: u64,
}
impl Defaults {
    fn default_timeout() -> u64 {
        3000
    }
}
impl Default for Defaults {
    fn default() -> Self {
        Self { timeout_ms_per_file: 3000 }
    }
}

#[derive(Deserialize)]
struct Plugin {
    cmd: String,
    args: Vec<String>,
}

#[derive(Serialize)]
struct Req<'a> {
    #[serde(rename = "type")]
    ty: &'static str,
    path: &'a str,
    text: &'a str,
}

#[derive(Deserialize)]
struct Resp {
    #[serde(rename = "type")]
    ty: String,
    violations: Vec<Violation>,
}

#[derive(Deserialize)]
struct Violation {
    rule_id: String,
    severity: Severity,
    message: String,
    location: Location,
}

#[derive(Deserialize)]
#[serde(rename_all = "lowercase")]
enum Severity {
    Error,
    Warning,
    Info,
}
impl std::fmt::Display for Severity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Severity::Error => write!(f, "error"),
            Severity::Warning => write!(f, "warning"),
            Severity::Info => write!(f, "info"),
        }
    }
}

#[allow(dead_code)]
#[derive(Deserialize)]
struct Location {
    line: u32,
    col: u32,
    end_line: u32,
    end_col: u32,
}

fn main() -> ExitCode {
    if let Err(e) = run_linter() {
        eprintln!("{}", e.to_string());
        return ExitCode::from(3);
    }
    ExitCode::SUCCESS
}

fn run_linter() -> Result<()> {
    let args = Args::parse();

    let (cfg_path, cfg_dir) = load_config_required(&args)?;
    let cfg_text = fs::read_to_string(&cfg_path)
        .with_context(|| format!("{E_CONFIG_NOT_FOUND}: {}", cfg_path.display()))?;
    let cfg: Config = toml::from_str(&cfg_text).map_err(|_| anyhow::anyhow!(E_INVALID_TOML))?;
    validate_config(&cfg)?;

    let input_bytes = fs::read(&args.input)
        .with_context(|| format!("failed to read input: {}", args.input.display()))?;
    let mut text =
        decode_utf8_with_bom(&input_bytes).map_err(|_| anyhow::anyhow!("invalid utf-8"))?;
    normalize_newlines(&mut text);
    ensure!(text.as_bytes().len() <= MAX_INPUT_BYTES, E_SIZE_OVER);

    let timeout_ms = cfg.defaults.timeout_ms_per_file.clamp(TIMEOUT_MIN_MS, TIMEOUT_MAX_MS);
    let timeout = Duration::from_millis(timeout_ms);

    let (cmd, argv) = resolve_plugin_cmd_and_args(&cfg_dir, &cfg.plugin);

    let mut child = Command::new(cmd)
        .args(argv)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .map_err(|_| anyhow::anyhow!("failed to spawn plugin"))?;

    {
        let mut stdin = child.stdin.take().ok_or_else(|| anyhow::anyhow!(E_PROTOCOL))?;
        let req = Req {
            ty: "CheckFileSingle",
            path: &args.input.to_string_lossy(),
            text: &text,
        };
        to_writer(&mut stdin, &req).map_err(|_| anyhow::anyhow!(E_PROTOCOL))?;
        stdin.write_all(b"\n").map_err(|_| anyhow::anyhow!(E_PROTOCOL))?;
        drop(stdin);
    }

    let mut stdout = BufReader::new(child.stdout.take().ok_or_else(|| anyhow::anyhow!(E_PROTOCOL))?);
    let (tx, rx) = mpsc::channel::<String>();
    thread::spawn(move || {
        let mut line = String::new();
        let _ = stdout.read_line(&mut line);
        let _ = tx.send(line);
    });

    let line = match rx.recv_timeout(timeout) {
        Ok(s) => s,
        Err(_) => {
            let _ = child.kill();
            let _ = child.wait();
            bail!(E_TIMEOUT);
        }
    };

    let resp_line = line.trim_end_matches(&['\r', '\n'][..]);
    if resp_line.is_empty() {
        bail!(E_PROTOCOL);
    }

    let resp: Resp =
        serde_json::from_str(resp_line).map_err(|_| anyhow::anyhow!(E_INVALID_RESP_JSON))?;
    if resp.ty != "ViolationsSingle" {
        bail!(E_PROTOCOL);
    }

    if resp.violations.is_empty() {
        return Ok(());
    }

    let display_path = to_display_path(&args.input);
    print_violations(&display_path, &text, &resp.violations)?;
    std::process::exit(2);
}

fn load_config_required(args: &Args) -> Result<(PathBuf, PathBuf)> {
    let cfg_path = if let Some(p) = &args.config {
        if !p.exists() {
            bail!("{E_CONFIG_NOT_FOUND_FLAG}: {}", p.display());
        }
        p.clone()
    } else {
        let p = PathBuf::from("sv-mint.toml");
        if !p.exists() {
            bail!("{E_CONFIG_NOT_FOUND}: {}", p.display());
        }
        p
    };
    let cfg_dir = cfg_path.parent().unwrap_or_else(|| Path::new(".")).to_path_buf();
    Ok((cfg_path, cfg_dir))
}

fn validate_config(cfg: &Config) -> Result<()> {
    ensure!(!cfg.plugin.cmd.trim().is_empty(), E_INVALID_TOML);
    ensure!(
        (TIMEOUT_MIN_MS..=TIMEOUT_MAX_MS).contains(&cfg.defaults.timeout_ms_per_file),
        E_INVALID_TOML
    );
    Ok(())
}

fn decode_utf8_with_bom(bytes: &[u8]) -> Result<String> {
    const BOM: &[u8] = &[0xEF, 0xBB, 0xBF];
    let s = if bytes.starts_with(BOM) {
        std::str::from_utf8(&bytes[BOM.len()..])?
    } else {
        std::str::from_utf8(bytes)?
    };
    Ok(s.to_string())
}

fn normalize_newlines(text: &mut String) {
    let mut out = String::with_capacity(text.len());
    let mut chars = text.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '\r' {
            if matches!(chars.peek(), Some(&'\n')) {
                let _ = chars.next();
            }
            out.push('\n');
        } else {
            out.push(c);
        }
    }
    *text = out;
}

fn resolve_plugin_cmd_and_args(cfg_dir: &Path, plugin: &Plugin) -> (String, Vec<String>) {
    let cmd = resolve_plugin_path(cfg_dir, &plugin.cmd);
    let argv: Vec<String> = plugin
        .args
        .iter()
        .map(|a| resolve_plugin_path(cfg_dir, a))
        .collect();
    (cmd, argv)
}

fn resolve_plugin_path(cfg_dir: &Path, token: &str) -> String {
    let p = Path::new(token);
    if p.has_root() {
        return token.to_string();
    }
    if p.components().count() > 1 || token.starts_with('.') {
        return cfg_dir.join(p).to_string_lossy().into_owned();
    }
    token.to_string()
}

fn to_display_path(p: &Path) -> String {
    let s = p.display().to_string();
    if let Some(rest) = s.strip_prefix(r"\\?\") {
        rest.to_string()
    } else {
        s
    }
}

fn print_violations(path: &str, text: &str, vs: &[Violation]) -> Result<()> {
    let lines: Vec<&str> = text.split('\n').collect();
    let mut out = std::io::BufWriter::new(std::io::stdout().lock());
    for v in vs {
        let lidx = v.location.line.saturating_sub(1) as usize;
        let excerpt = lines.get(lidx).copied().unwrap_or_default();
        writeln!(
            &mut out,
            "{}:{}:{}: [{}] {}: {}",
            path, v.location.line, v.location.col, v.severity, v.rule_id, v.message
        )?;
        writeln!(&mut out, "    > {}", excerpt)?;
    }
    out.flush()?;
    Ok(())
}
