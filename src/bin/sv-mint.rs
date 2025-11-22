use clap::Parser;
use std::path::PathBuf;
use std::process::ExitCode;

use sv_mint::config::{apply_rule_overrides, load_from_path};
use sv_mint::core::pipeline::Pipeline;
use sv_mint::diag::logging::init as log_init;
use sv_mint::errors::ConfigError;

#[derive(Parser, Debug)]
#[command(
    name = "sv-mint",
    version,
    about = "SystemVerilog linter (Windows, sv-parser integrated)"
)]
struct Cli {
    #[arg(long, value_name = "CONFIG")]
    config: Option<PathBuf>,
    #[arg(long, value_name = "RULE", value_delimiter = ',')]
    disable: Vec<String>,
    #[arg(long, value_name = "RULE", value_delimiter = ',')]
    only: Vec<String>,
    #[arg(
        value_name = "INPUT",
        num_args = 1..
    )]
    input: Vec<PathBuf>,
}

fn main() -> ExitCode {
    let cli = Cli::parse();

    let (mut cfg, _) = match load_from_path(cli.config) {
        Ok(pair) => pair,
        Err(e) => {
            eprintln!("{}", e);
            return ExitCode::from(3);
        }
    };

    if let Err(e) = log_init(&cfg.logging) {
        eprintln!("{}", e);
        return ExitCode::from(3);
    }

    if let Err(e) = apply_rule_overrides(&mut cfg.rule, &cli.only, &cli.disable) {
        eprintln!("{}", e);
        return ExitCode::from(3);
    }

    let inputs = match gather_inputs(&cli.input) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("{}", e);
            return ExitCode::from(3);
        }
    };

    let pipeline = Pipeline::new(&cfg);
    let summary = match pipeline.run_files(&inputs) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("{}", e);
            return ExitCode::from(3);
        }
    };

    summary.exit_code()
}

fn gather_inputs(direct: &[PathBuf]) -> Result<Vec<PathBuf>, ConfigError> {
    if direct.is_empty() {
        return Err(ConfigError::InvalidValue {
            detail: "no input files provided".to_string(),
        });
    }
    Ok(direct.to_vec())
}
