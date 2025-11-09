mod config;
mod output;
mod plugin;
mod svparser;
mod textutil;
mod types;

use anyhow::{Context, Result};
use clap::Parser;
use serde_json::json;
use std::fs;
use std::path::PathBuf;
use std::process::ExitCode;
use types::Stage;

#[derive(Parser, Debug)]
#[command(version, author, about = "sv-mint: SystemVerilog linter")]
struct Cli {
    #[arg(long = "config")]
    config: Option<PathBuf>,
    #[arg()]
    input: PathBuf,
}

fn main() -> ExitCode {
    match run() {
        Ok(code) => code,
        Err(e) => {
            ExitCode::from(3)
        }
    }
}

fn run() -> Result<ExitCode> {
    let cli = Cli::parse();

    let cfg_path = config::resolve_path(cli.config)?;
    let cfg_dir = cfg_path
        .parent()
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("."));
    let cfg_text = fs::read_to_string(&cfg_path).context("failed to read config file")?;
    let cfg = config::load(&cfg_text).context("failed to parse config")?;
    config::validate_config(&cfg).context("invalid config")?;

    let (normalized_text, input_path) = config::read_input(&cli.input).context("failed to read input file")?;
    let (pp_text, final_defs, cst_opt) =
        svparser::run_svparser(&input_path, &cfg_dir, &cfg.svparser).context("svparser failed")?;

    let mut all = Vec::new();
    for stage in &cfg.stages.enabled {
        let payload = match stage {
            Stage::RawText => json!({ "text": &normalized_text }),
            Stage::PpText => svparser::build_pp_payload(&cfg, &pp_text, &final_defs),
            Stage::Cst => svparser::build_cst_payload(&cst_opt),
            Stage::Ast => svparser::build_ast_payload(&input_path, &pp_text, &cst_opt),
        };
        let vs = plugin::run_plugin_once(&cfg_dir, &cfg, stage.as_str(), &input_path, payload)?;
        all.extend(vs);
    }

    output::print_violations(&input_path, &all);
    let code = if all.is_empty() { 0 } else { 2 };
    Ok(ExitCode::from(code))
}
