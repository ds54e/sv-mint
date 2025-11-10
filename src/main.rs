mod config;
mod errors;
mod output;
mod plugin;
mod svparser;
mod textutil;
mod types;

use crate::errors::AppResult;
use clap::Parser;
use config::validate_config;
use log::{error, info};
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
        Ok(n_violations) => {
            if n_violations == 0 {
                ExitCode::from(0)
            } else {
                ExitCode::from(2)
            }
        }
        Err(e) => {
            eprintln!("{}", e);
            error!("event=error msg={}", e);
            ExitCode::from(3)
        }
    }
}

fn run() -> AppResult<usize> {
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
    let (pp_text, final_defs, cst_opt) =
        run_svparser(&input_path, &cfg_dir, &cfg.svparser, cfg.logging.show_parse_events)?;

    let mut all_violations = Vec::new();
    for stage in &cfg.stages.enabled {
        if cfg.logging.show_stage_events {
            info!(
                "event=stage_start stage={} path={}",
                stage.as_str(),
                input_path.display()
            );
        }
        let payload = match stage {
            Stage::RawText => json!({ "text": &normalized_text }),
            Stage::PpText => svparser::build_pp_payload(&cfg, &pp_text, &final_defs),
            Stage::Cst => svparser::build_cst_payload(&cst_opt),
            Stage::Ast => svparser::build_ast_payload(&input_path, &pp_text, &cst_opt, cfg.logging.show_parse_events),
        };
        let vs = run_plugin_once(&cfg_dir, &cfg, stage.as_str(), &input_path, payload)?;
        all_violations.extend(vs);
        if cfg.logging.show_stage_events {
            info!(
                "event=stage_done stage={} path={}",
                stage.as_str(),
                input_path.display()
            );
        }
    }

    print_violations(Path::new(&input_path), &all_violations);
    Ok(all_violations.len())
}
