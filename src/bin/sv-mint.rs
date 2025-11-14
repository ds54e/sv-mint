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
    use std::collections::{HashSet, VecDeque};
    const MAX_FILES: usize = 50_000;
    let mut seen = HashSet::new();
    let mut out = Vec::new();
    let ext_set: Vec<String> = lib_exts
        .iter()
        .map(|ext| {
            if ext.starts_with('.') {
                ext.to_ascii_lowercase()
            } else {
                format!(".{}", ext).to_ascii_lowercase()
            }
        })
        .collect();
    for dir in lib_dirs {
        let mut queue: VecDeque<PathBuf> = VecDeque::new();
        queue.push_back(dir.clone());
        while let Some(current) = queue.pop_front() {
            let meta = std::fs::symlink_metadata(&current).map_err(|_| ConfigError::NotFound {
                path: current.display().to_string(),
            })?;
            if meta.file_type().is_symlink() {
                continue;
            }
            if meta.is_dir() {
                let entries = match std::fs::read_dir(&current) {
                    Ok(iter) => iter,
                    Err(_) => continue,
                };
                for entry in entries.flatten() {
                    queue.push_back(entry.path());
                }
                continue;
            }
            if meta.is_file() {
                if let Some(ext) = current.extension().and_then(|e| e.to_str()) {
                    let dot_ext = format!(".{}", ext.to_ascii_lowercase());
                    if ext_set.iter().any(|needle| needle == &dot_ext)
                        && seen.insert(current.clone())
                    {
                        out.push(current.clone());
                        if out.len() >= MAX_FILES {
                            return Err(ConfigError::InvalidValue {
                                detail: format!(
                                    "libext auto-discovery exceeded {} files; tighten -y/+libext scope",
                                    MAX_FILES
                                ),
                            });
                        }
                    }
                }
            }
        }
    }
    Ok(out)
}
