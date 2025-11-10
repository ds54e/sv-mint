mod config;
mod output;
mod plugin;
mod svparser;
mod textutil;
mod types;

use anyhow::Result;
use clap::Parser;
use config::validate_config;
use output::print_violations;
use plugin::run_plugin_once;
use serde_json::json;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::ExitCode;
use svparser::run_svparser;
use types::Stage;

#[derive(Parser, Debug)]
#[command(
    version,
    author,
    about = "sv-mint: SystemVerilog linter (Windows, sv-parser integrated)"
)]
struct Cli {
    #[arg(long = "config", help = "Path to sv-mint.toml")]
    config: Option<PathBuf>,
    #[arg(help = "Path to input SystemVerilog file")]
    input: PathBuf,
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

    let cfg_path = config::resolve_path(cli.config)?;
    let cfg_dir = cfg_path
        .parent()
        .map(|p| p.to_path_buf())
        .unwrap_or_else(|| PathBuf::from("."));

    let cfg_text = fs::read_to_string(&cfg_path)?;
    let cfg = config::load(&cfg_text)?;
    validate_config(&cfg)?;

    let (normalized_text, input_path) = config::read_input(&cli.input)?;
    let (pp_text, final_defs, cst_opt) = run_svparser(&input_path, &cfg_dir, &cfg.svparser)?;

    let mut all_violations = Vec::new();
    for stage in &cfg.stages.enabled {
        let payload = match stage {
            Stage::RawText => json!({ "text": &normalized_text }),
            Stage::PpText => svparser::build_pp_payload(&cfg, &pp_text, &final_defs),
            Stage::Cst => svparser::build_cst_payload(&cst_opt),
            Stage::Ast => svparser::build_ast_payload(&input_path, &pp_text, &cst_opt),
        };
        let vs = run_plugin_once(&cfg_dir, &cfg, stage.as_str(), &input_path, payload)?;
        all_violations.extend(vs);
    }

    print_violations(Path::new(&input_path), &all_violations);
    Ok(if all_violations.is_empty() {
        ExitCode::from(0)
    } else {
        ExitCode::from(2)
    })
}
