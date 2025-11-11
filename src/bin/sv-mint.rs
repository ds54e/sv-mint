use clap::Parser;
use std::path::PathBuf;
use std::process::ExitCode;

use sv_mint::config::{load, resolve_path, validate_config, Config};
use sv_mint::core::pipeline::Pipeline;
use sv_mint::diag::logging::init as log_init;

#[derive(Parser, Debug)]
#[command(
    name = "sv-mint",
    version,
    about = "SystemVerilog linter (Windows, sv-parser integrated)"
)]
struct Cli {
    #[arg(long, value_name = "CONFIG")]
    config: Option<PathBuf>,
    #[arg(value_name = "INPUT", required = true)]
    input: Vec<PathBuf>,
}

fn main() -> ExitCode {
    let cli = Cli::parse();

    let cfg_path = match resolve_path(cli.config) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("{}", e);
            return ExitCode::from(3);
        }
    };

    let cfg_text = match std::fs::read_to_string(&cfg_path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("{}", e);
            return ExitCode::from(3);
        }
    };

    let cfg: Config = match load(&cfg_text) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("{}", e);
            return ExitCode::from(3);
        }
    };

    if let Err(e) = validate_config(&cfg) {
        eprintln!("{}", e);
        return ExitCode::from(3);
    }

    if let Err(e) = log_init(&cfg.logging) {
        eprintln!("{}", e);
        return ExitCode::from(3);
    }

    let pipeline = Pipeline::new(&cfg);
    let summary = match pipeline.run_files(&cli.input) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("{}", e);
            return ExitCode::from(3);
        }
    };

    if summary.had_error {
        ExitCode::from(3)
    } else if summary.violations > 0 {
        ExitCode::from(2)
    } else {
        ExitCode::from(0)
    }
}
