use crate::svparser::SvParserCfg;
use crate::textutil::{normalize_lf, strip_bom};
use anyhow::{anyhow, ensure, Result};
use env_logger::Builder;
use log::LevelFilter;
use serde::Deserialize;
use serde_json::Value;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(serde::Deserialize, Clone)]
pub struct LoggingConfig {
    pub level: String,
    pub stderr_snippet_bytes: usize,
    pub show_stage_events: bool,
    pub show_plugin_events: bool,
    pub show_parse_events: bool,
}

impl Default for LoggingConfig {
    fn default() -> Self {
        Self {
            level: "info".to_string(),
            stderr_snippet_bytes: 2048,
            show_stage_events: true,
            show_plugin_events: true,
            show_parse_events: true,
        }
    }
}

#[derive(Deserialize)]
pub struct Config {
    #[serde(default)]
    pub logging: LoggingConfig,
    pub defaults: Defaults,
    pub plugin: Plugin,
    pub stages: Stages,
    #[serde(default)]
    pub svparser: SvParserCfg,
    #[serde(default)]
    pub rules: Value,
}

#[derive(Deserialize)]
pub struct Defaults {
    pub timeout_ms_per_file: u64,
}

#[derive(Deserialize)]
pub struct Plugin {
    pub cmd: String,
    #[serde(default)]
    pub args: Vec<String>,
}

#[derive(Deserialize)]
pub struct Stages {
    pub enabled: Vec<crate::types::Stage>,
}

pub const E_CONFIG_NOT_FOUND: &str = "config not found";
pub const E_INVALID_TOML: &str = "invalid toml";

pub fn resolve_path(opt: Option<PathBuf>) -> Result<PathBuf> {
    match opt {
        Some(p) if p.exists() => Ok(p),
        Some(p) => Err(anyhow!("{E_CONFIG_NOT_FOUND} (from --config): {}", p.display())),
        None => {
            let p = PathBuf::from("sv-mint.toml");
            ensure!(p.exists(), "{}: {}", E_CONFIG_NOT_FOUND, p.display());
            Ok(p)
        }
    }
}

pub fn load(cfg_text: &str) -> Result<Config> {
    let cfg: Config = toml::from_str(cfg_text).map_err(|_| anyhow!(E_INVALID_TOML))?;
    Ok(cfg)
}

pub fn read_input(path: &Path) -> Result<(String, PathBuf)> {
    let raw = fs::read(path)?;
    ensure!(std::str::from_utf8(&raw).is_ok(), "invalid utf-8");
    let text = normalize_lf(strip_bom(String::from_utf8(raw)?));
    Ok((text, path.to_path_buf()))
}

pub fn validate_config(cfg: &Config) -> Result<()> {
    const TIMEOUT_MIN_MS: u64 = 100;
    const TIMEOUT_MAX_MS: u64 = 60_000;
    ensure!(
        (TIMEOUT_MIN_MS..=TIMEOUT_MAX_MS).contains(&cfg.defaults.timeout_ms_per_file),
        "timeout out of range"
    );
    ensure!(!cfg.plugin.cmd.trim().is_empty(), "plugin cmd empty");
    let mut b = Builder::new();
    let lvl = match cfg.logging.level.as_str() {
        "error" => LevelFilter::Error,
        "warn" => LevelFilter::Warn,
        "debug" => LevelFilter::Debug,
        "info" => LevelFilter::Info,
        _ => LevelFilter::Info,
    };
    b.filter_level(lvl);
    let _ = b.try_init();
    let _ = (
        cfg.logging.stderr_snippet_bytes,
        cfg.logging.show_stage_events,
        cfg.logging.show_plugin_events,
        cfg.logging.show_parse_events,
    );
    Ok(())
}
