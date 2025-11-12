use crate::errors::ConfigError;
use crate::svparser::SvParserCfg;
use crate::textutil::{normalize_lf, strip_bom};
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
    #[serde(default)]
    pub ruleset: Ruleset,
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

#[derive(Deserialize, Clone, Default)]
pub struct Ruleset {
    pub scripts: Vec<String>,
    #[serde(default, rename = "override")]
    pub severity_override: std::collections::HashMap<String, String>,
}

pub fn resolve_path(opt: Option<PathBuf>) -> Result<PathBuf, ConfigError> {
    match opt {
        Some(p) if p.exists() => Ok(p),
        Some(p) => Err(ConfigError::NotFound {
            path: p.display().to_string(),
        }),
        None => {
            let p = PathBuf::from("sv-mint.toml");
            if !p.exists() {
                return Err(ConfigError::NotFound {
                    path: p.display().to_string(),
                });
            }
            Ok(p)
        }
    }
}

pub fn load(cfg_text: &str) -> Result<Config, ConfigError> {
    toml::from_str(cfg_text).map_err(|e| ConfigError::InvalidToml { detail: e.to_string() })
}

pub fn read_input(path: &Path) -> Result<(String, PathBuf), ConfigError> {
    let raw = fs::read(path).map_err(|_| ConfigError::NotFound {
        path: path.display().to_string(),
    })?;
    if std::str::from_utf8(&raw).is_err() {
        return Err(ConfigError::InvalidUtf8 {
            path: path.display().to_string(),
            source: None,
        });
    }
    let text = normalize_lf(strip_bom(String::from_utf8(raw).map_err(|_| {
        ConfigError::InvalidUtf8 {
            path: path.display().to_string(),
            source: None,
        }
    })?));
    Ok((text, path.to_path_buf()))
}

pub fn validate_config(cfg: &Config) -> Result<(), ConfigError> {
    const TIMEOUT_MIN_MS: u64 = 100;
    const TIMEOUT_MAX_MS: u64 = 60_000;
    if !(TIMEOUT_MIN_MS..=TIMEOUT_MAX_MS).contains(&cfg.defaults.timeout_ms_per_file) {
        return Err(ConfigError::InvalidValue {
            detail: "timeout out of range".to_string(),
        });
    }
    if cfg.plugin.cmd.trim().is_empty() {
        return Err(ConfigError::InvalidValue {
            detail: "plugin cmd empty".to_string(),
        });
    }
    if cfg.stages.enabled.is_empty() {
        return Err(ConfigError::InvalidValue {
            detail: "stages.enabled empty".to_string(),
        });
    }
    Ok(())
}
