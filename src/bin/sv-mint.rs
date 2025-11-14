use clap::Parser;
use std::path::PathBuf;
use std::process::ExitCode;

use sv_mint::config::{apply_rule_overrides, load_from_path, Config};
use sv_mint::core::pipeline::Pipeline;
use sv_mint::diag::logging::init as log_init;
use sv_mint::errors::ConfigError;
use sv_mint::filelist::load_filelists;

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
    #[arg(short = 'f', long = "filelist", value_name = "FILELIST")]
    filelist: Vec<PathBuf>,
    #[arg(
        value_name = "INPUT",
        required_unless_present_any = ["filelist"],
        num_args = 0..
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

    let inputs = match gather_inputs(&mut cfg, &cli.input, &cli.filelist) {
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

    if summary.had_error {
        ExitCode::from(3)
    } else if summary.violations > 0 {
        ExitCode::from(2)
    } else {
        ExitCode::from(0)
    }
}

fn gather_inputs(cfg: &mut Config, direct: &[PathBuf], filelists: &[PathBuf]) -> Result<Vec<PathBuf>, ConfigError> {
    let mut inputs: Vec<PathBuf> = direct.to_vec();
    let mut lib_dirs: Vec<PathBuf> = Vec::new();
    let mut lib_exts: Vec<String> = Vec::new();
    if !filelists.is_empty() {
        let load = load_filelists(filelists)?;
        inputs.extend(load.files);
        for inc in load.incdirs {
            cfg.svparser.include_paths.push(inc.to_string_lossy().into_owned());
        }
        for define in load.defines {
            cfg.svparser.defines.push(define);
        }
        lib_dirs = load.lib_dirs;
        lib_exts = load.libexts;
    }
    if !lib_dirs.is_empty() && !lib_exts.is_empty() {
        let discovered = discover_lib_files(&lib_dirs, &lib_exts)?;
        inputs.extend(discovered);
    }
    if inputs.is_empty() {
        return Err(ConfigError::InvalidValue {
            detail: "no input files provided".to_string(),
        });
    }
    Ok(inputs)
}

fn discover_lib_files(lib_dirs: &[PathBuf], lib_exts: &[String]) -> Result<Vec<PathBuf>, ConfigError> {
    use std::collections::HashSet;
    let mut seen = HashSet::new();
    let mut out = Vec::new();
    for dir in lib_dirs {
        let entries = std::fs::read_dir(dir).map_err(|_| ConfigError::NotFound {
            path: dir.display().to_string(),
        })?;
        for entry in entries.flatten() {
            let path = entry.path();
            if !path.is_file() {
                continue;
            }
            if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                let dot_ext = format!(".{}", ext);
                if lib_exts.iter().any(|needle| needle.eq_ignore_ascii_case(&dot_ext)) {
                    if seen.insert(path.clone()) {
                        out.push(path);
                    }
                }
            }
        }
    }
    Ok(out)
}
