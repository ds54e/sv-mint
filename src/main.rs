use anyhow::{anyhow, bail, ensure, Result};
use clap::Parser;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;
use std::fs;
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode, Stdio};
use std::sync::mpsc;
use std::time::Duration;
use sv_parser::{parse_sv, preprocess, Define, DefineText, Defines, SyntaxTree};

const MAX_INPUT_BYTES: usize = 8 * 1024 * 1024;
const TIMEOUT_MIN_MS: u64 = 100;
const TIMEOUT_MAX_MS: u64 = 60_000;

const E_CONFIG_NOT_FOUND: &str = "config not found";
const E_INVALID_TOML: &str = "invalid toml";
const E_INVALID_UTF8: &str = "invalid utf-8";
const E_SIZE_OVER: &str = "size over";
const E_TIMEOUT: &str = "timeout";
const E_INVALID_JSON: &str = "invalid response json";
const E_PROTOCOL: &str = "protocol error";
const E_PREPROCESS_FAILED: &str = "preprocess failed";
const E_PARSING_FAILED: &str = "parsing failed";

#[derive(Parser, Debug)]
#[command(
    version,
    author,
    about = "sv-mint: SystemVerilog linter (, Windows, sv-parser integrated)"
)]
struct Cli {
    #[arg(long = "config", help = "Path to sv-mint.toml")]
    config: Option<PathBuf>,
    #[arg(help = "Path to input SystemVerilog file")]
    input: PathBuf,
}

#[derive(Deserialize)]
struct Config {
    defaults: Defaults,
    plugin: Plugin,
    stages: Stages,
    #[serde(default)]
    svparser: SvParserCfg,
    #[serde(default)]
    rules: Value,
}

#[derive(Deserialize)]
struct Defaults {
    timeout_ms_per_file: u64,
}

#[derive(Deserialize)]
struct Plugin {
    cmd: String,
    #[serde(default)]
    args: Vec<String>,
}

#[derive(Deserialize)]
struct Stages {
    enabled: Vec<String>,
}

#[derive(Deserialize, Default)]
struct SvParserCfg {
    #[serde(default)]
    include_paths: Vec<String>,
    #[serde(default)]
    defines: Vec<String>,
    #[serde(default)]
    strip_comments: bool,
    #[serde(default)]
    ignore_include: bool,
    #[serde(default)]
    allow_incomplete: bool,
}

#[derive(Serialize, Deserialize)]
struct Location {
    line: u32,
    col: u32,
    end_line: u32,
    end_col: u32,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
enum Severity {
    Error,
    Warning,
    Info,
}

#[derive(Serialize, Deserialize)]
struct Violation {
    rule_id: String,
    severity: Severity,
    message: String,
    location: Location,
}

#[derive(Serialize)]
struct Req<'a> {
    #[serde(rename = "type")]
    ty: &'static str,
    stage: &'a str,
    path: &'a str,
    payload: Value,
}

#[derive(Deserialize)]
struct Resp {
    #[serde(rename = "type")]
    ty: String,
    stage: String,
    violations: Vec<Violation>,
}

fn main() -> ExitCode {
    match run() {
        Ok(code) => code,
        Err(e) => {
            eprintln!("{}", e);
            ExitCode::from(3)
        }
    }
}

fn run() -> Result<ExitCode> {
    let cli = Cli::parse();

    let cfg_path = match &cli.config {
        Some(p) => {
            ensure!(p.exists(), "{E_CONFIG_NOT_FOUND} (from --config): {}", p.display());
            p.clone()
        }
        None => {
            let p = PathBuf::from("sv-mint.toml");
            ensure!(p.exists(), "{}: {}", E_CONFIG_NOT_FOUND, p.display());
            p
        }
    };

    let cfg_dir = cfg_path
        .parent()
        .map(Path::to_path_buf)
        .unwrap_or_else(|| PathBuf::from("."));
    let cfg_text = fs::read_to_string(&cfg_path).map_err(|_| anyhow!(E_INVALID_TOML))?;
    let cfg: Config = toml::from_str(&cfg_text).map_err(|_| anyhow!(E_INVALID_TOML))?;

    validate_config(&cfg)?;

    let input_path = cli.input.clone();
    let raw_bytes = fs::read(&input_path).map_err(|_| anyhow!("read input failed"))?;
    ensure!(std::str::from_utf8(&raw_bytes).is_ok(), E_INVALID_UTF8);
    let normalized_text = normalize_lf(strip_bom(String::from_utf8(raw_bytes).unwrap()));
    ensure!(normalized_text.len() <= MAX_INPUT_BYTES, E_SIZE_OVER);

    let (pp_text, final_defs, cst_opt) = run_svparser(
        &input_path,
        &cfg_dir,
        &cfg.svparser,
        &normalized_text,
        cfg.stages.enabled.iter().any(|s| s == "pp_text"),
        cfg.stages.enabled.iter().any(|s| s == "cst"),
    )?;

    let mut all_violations: Vec<Violation> = Vec::new();
    for stage in &cfg.stages.enabled {
        match stage.as_str() {
            "raw_text" => {
                let payload = json!({ "text": normalized_text });
                let vs = run_plugin_once(&cfg_dir, &cfg, "raw_text", &input_path, payload)?;
                all_violations.extend(vs);
            }
            "pp_text" => {
                let payload = build_pp_payload(&cfg, &pp_text, &final_defs);
                let vs = run_plugin_once(&cfg_dir, &cfg, "pp_text", &input_path, payload)?;
                all_violations.extend(vs);
            }
            "cst" => {
                let cst_json = cst_opt
                    .as_ref()
                    .map(|_| {
                        json!({
                            "kind":"SyntaxTree",
                            "range":{"line":1,"col":1,"end_line":1,"end_col":1},
                            "children":[]
                        })
                    })
                    .unwrap_or_else(|| {
                        json!({
                            "kind":"Empty",
                            "range":{"line":1,"col":1,"end_line":1,"end_col":1},
                            "children":[]
                        })
                    });
                let payload = json!({ "cst": cst_json });
                let vs = run_plugin_once(&cfg_dir, &cfg, "cst", &input_path, payload)?;
                all_violations.extend(vs);
            }
            _ => {}
        }
    }

    let path_for_print = strip_unc_prefix(&input_path);
    print_violations(&path_for_print, &normalized_text, &all_violations)?;

    Ok(if all_violations.is_empty() {
        ExitCode::from(0)
    } else {
        ExitCode::from(2)
    })
}

fn validate_config(cfg: &Config) -> Result<()> {
    ensure!(
        (TIMEOUT_MIN_MS..=TIMEOUT_MAX_MS).contains(&cfg.defaults.timeout_ms_per_file),
        "timeout out of range"
    );
    ensure!(!cfg.plugin.cmd.trim().is_empty(), "plugin cmd empty");
    for s in &cfg.stages.enabled {
        ensure!(s == "raw_text" || s == "pp_text" || s == "cst", "unknown stage: {}", s);
    }
    Ok(())
}

fn run_svparser(
    input_path: &Path,
    cfg_dir: &Path,
    sp: &SvParserCfg,
    normalized_text: &str,
    need_pp: bool,
    need_cst: bool,
) -> Result<(String, Defines, Option<SyntaxTree>)> {
    if !need_pp && !need_cst {
        return Ok((String::new(), Defines::new(), None));
    }

    let incs: Vec<String> = sp
        .include_paths
        .iter()
        .map(|p| make_abs(cfg_dir, p).display().to_string())
        .collect();

    let mut pre_defines: HashMap<String, Option<Define>> = HashMap::new();
    for d in &sp.defines {
        if let Some((n, v)) = d.split_once('=') {
            let body = DefineText::new(v.to_string(), None);
            pre_defines.insert(n.to_string(), Some(Define::new(n.to_string(), Vec::new(), Some(body))));
        } else {
            pre_defines.insert(d.to_string(), None);
        }
    }

    let final_defs = if need_pp {
        let (_pp_text_obj, final_defs) =
            preprocess(input_path, &pre_defines, &incs, sp.strip_comments, sp.ignore_include)
                .map_err(|_| anyhow!(E_PREPROCESS_FAILED))?;
        final_defs
    } else {
        Defines::new()
    };

    let cst = if need_cst {
        let (tree, _defs2) = parse_sv(input_path, &pre_defines, &incs, sp.ignore_include, sp.allow_incomplete)
            .map_err(|_| anyhow!(E_PARSING_FAILED))?;
        Some(tree)
    } else {
        None
    };

    let pp_text_str = if need_pp {
        normalized_text.to_string()
    } else {
        String::new()
    };
    Ok((pp_text_str, final_defs, cst))
}

fn build_pp_payload(cfg: &Config, pp_text: &str, final_defs: &Defines) -> Value {
    let mut names: Vec<String> = final_defs.keys().cloned().collect();
    names.sort();

    let mut predefined_names: Vec<String> = Vec::new();
    for d in cfg
        .svparser
        .defines
        .iter()
        .filter_map(|s| s.split_once('=').map(|(n, _)| n).or(Some(s.as_str())))
    {
        predefined_names.push(d.to_string());
    }
    predefined_names.sort();
    predefined_names.dedup();

    json!({
        "text": pp_text,
        "include_paths": cfg.svparser.include_paths,
        "defines": cfg.svparser.defines,
        "defines_table": names,
        "predefined_names": predefined_names,
        "rules": cfg.rules
    })
}

fn run_plugin_once(
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
        let mut stdin = child.stdin.take().ok_or_else(|| anyhow!(E_PROTOCOL))?;
        let req = Req {
            ty: "CheckFileStage",
            stage,
            path: &input_path.to_string_lossy(),
            payload,
        };
        serde_json::to_writer(&mut stdin, &req).map_err(|_| anyhow!(E_PROTOCOL))?;
        stdin.write_all(b"\n").map_err(|_| anyhow!(E_PROTOCOL))?;
    }

    let mut stdout = BufReader::new(child.stdout.take().ok_or_else(|| anyhow!(E_PROTOCOL))?);
    let (tx, rx) = mpsc::channel::<String>();
    std::thread::spawn(move || {
        let mut line = String::new();
        let _ = stdout.read_line(&mut line);
        let _ = tx.send(line);
    });

    let to = Duration::from_millis(cfg.defaults.timeout_ms_per_file.clamp(TIMEOUT_MIN_MS, TIMEOUT_MAX_MS));
    let line = rx.recv_timeout(to).map_err(|_| anyhow!(E_TIMEOUT))?;
    if line.trim().is_empty() {
        bail!(E_INVALID_JSON);
    }
    let resp: Resp = serde_json::from_str(&line).map_err(|_| anyhow!(E_INVALID_JSON))?;
    ensure!(resp.ty == "ViolationsStage", E_PROTOCOL);
    ensure!(resp.stage == stage, E_PROTOCOL);
    Ok(resp.violations)
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
            path,
            v.location.line,
            v.location.col,
            sev_str(&v.severity),
            v.rule_id,
            v.message
        )?;
        writeln!(&mut out, "    > {}", excerpt)?;
    }
    out.flush()?;
    Ok(())
}

fn sev_str(s: &Severity) -> &'static str {
    match s {
        Severity::Error => "error",
        Severity::Warning => "warning",
        Severity::Info => "info",
    }
}

fn strip_unc_prefix(p: &Path) -> String {
    let s = p.to_string_lossy();
    if let Some(stripped) = s.strip_prefix(r"\\?\") {
        stripped.to_string()
    } else {
        s.to_string()
    }
}

fn resolve_plugin_cmd_and_args(cfg_dir: &Path, plugin: &Plugin) -> (String, Vec<String>) {
    let mut argv = Vec::new();
    for a in &plugin.args {
        if looks_like_path(a) {
            argv.push(make_abs(cfg_dir, a).to_string_lossy().to_string());
        } else {
            argv.push(a.clone());
        }
    }
    (plugin.cmd.clone(), argv)
}

fn looks_like_path(s: &str) -> bool {
    s.contains(std::path::is_separator) || s.ends_with(".py") || s.ends_with(".exe")
}

fn make_abs(base: &Path, rel_or_abs: &str) -> PathBuf {
    let p = PathBuf::from(rel_or_abs);
    if p.is_absolute() {
        p
    } else {
        base.join(p)
    }
}

fn strip_bom(mut s: String) -> String {
    if s.as_bytes().starts_with(&[0xEF, 0xBB, 0xBF]) {
        s.drain(..3);
    }
    s
}

fn normalize_lf(s: String) -> String {
    s.replace("\r\n", "\n").replace('\r', "\n")
}
