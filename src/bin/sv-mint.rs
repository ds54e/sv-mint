use clap::Parser;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use sv_mint::config::{load, read_input, resolve_path, validate_config, Config};
use sv_mint::output::print_violations;
use sv_mint::plugin::run_plugin_once;
use sv_mint::types::{Stage, Violation};

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

fn run_for_one(input: &Path, cfg: &Config) -> anyhow::Result<usize> {
    let (normalized_text, input_path) = read_input(input)?;
    let mut all: Vec<Violation> = Vec::new();

    for stage in &cfg.stages.enabled {
        let payload = match stage {
            Stage::RawText => serde_json::json!({ "text": normalized_text }),
            Stage::PpText => serde_json::json!({ "text": normalized_text, "defines": [] }),
            Stage::Cst => serde_json::json!({ "has_cst": false }),
            Stage::Ast => serde_json::json!({ "decls": [], "refs": [], "symbols": [] }),
        };
        let vs = run_plugin_once(stage.as_str(), &input_path, payload)?;
        all.extend(vs);
    }

    print_violations(&all, &input_path);
    Ok(all.len())
}

fn main() -> ExitCode {
    if let Err(code) = real_main() {
        return code;
    }
    ExitCode::from(0)
}

fn real_main() -> Result<(), ExitCode> {
    let cli = Cli::parse();
    let cfg_path = resolve_path(cli.config).map_err(|_| ExitCode::from(3))?;
    let cfg_text = std::fs::read_to_string(&cfg_path).map_err(|_| ExitCode::from(3))?;
    let cfg = load(&cfg_text).map_err(|_| ExitCode::from(3))?;
    validate_config(&cfg).map_err(|_| ExitCode::from(3))?;

    let mut had_error = false;
    let mut n_viol: usize = 0;

    for inp in &cli.input {
        match run_for_one(inp, &cfg) {
            Ok(n) => n_viol += n,
            Err(_) => had_error = true,
        }
    }

    if had_error {
        Err(ExitCode::from(3))
    } else if n_viol > 0 {
        Err(ExitCode::from(2))
    } else {
        Ok(())
    }
}
