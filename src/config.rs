use crate::svparser::SvParserCfg;
use crate::textutil::{normalize_lf, strip_bom};
use anyhow::{anyhow, ensure, Result};
use serde::Deserialize;
use serde_json::Value;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Deserialize)]
pub struct Config {
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
    Ok(())
}
