use crate::errors::ConfigError;
use crate::svparser::SvParserCfg;
use crate::types::Stage;
use crate::textutil::{normalize_lf, strip_bom};
use serde::Deserialize;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};
use toml::Value as TomlValue;

#[derive(serde::Deserialize, Clone)]
pub struct LoggingConfig {
    pub level: String,
    pub stderr_snippet_bytes: usize,
    pub show_stage_events: bool,
    pub show_plugin_events: bool,
    pub show_parse_events: bool,
    #[serde(default)]
    pub format: LogFormat,
    #[serde(flatten, default)]
    pub extra: HashMap<String, TomlValue>,
}

impl Default for LoggingConfig {
    fn default() -> Self {
        Self {
            level: "info".to_string(),
            stderr_snippet_bytes: 2048,
            show_stage_events: true,
            show_plugin_events: true,
            show_parse_events: true,
            format: LogFormat::Text,
            extra: HashMap::new(),
        }
    }
}

#[derive(Deserialize, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum LogFormat {
    Text,
    Json,
}

impl Default for LogFormat {
    fn default() -> Self {
        LogFormat::Text
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
    pub rule: Vec<RuleConfig>,
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

#[derive(Deserialize, Clone)]
pub struct RuleConfig {
    pub id: String,
    pub script: String,
    pub stage: Stage,
    #[serde(default = "default_true")]
    pub enabled: bool,
    #[serde(default)]
    pub severity: Option<String>,
}

fn default_true() -> bool {
    true
}

pub fn apply_rule_overrides(
    rules: &mut [RuleConfig],
    only: &[String],
    disable: &[String],
) -> Result<(), ConfigError> {
    if only.is_empty() && disable.is_empty() {
        return Ok(());
    }
    let existing: HashSet<String> = rules.iter().map(|r| r.id.clone()).collect();
    for id in only.iter().chain(disable.iter()) {
        if !existing.contains(id) {
            return Err(ConfigError::InvalidValue {
                detail: format!("rule {} not found", id),
            });
        }
    }
    if !only.is_empty() {
        let only_set: HashSet<&str> = only.iter().map(|s| s.as_str()).collect();
        for rule in rules.iter_mut() {
            rule.enabled = only_set.contains(rule.id.as_str());
        }
    }
    if !disable.is_empty() {
        let disable_set: HashSet<&str> = disable.iter().map(|s| s.as_str()).collect();
        for rule in rules.iter_mut() {
            if disable_set.contains(rule.id.as_str()) {
                rule.enabled = false;
            }
        }
    }
    Ok(())
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

pub fn load_from_path(opt: Option<PathBuf>) -> Result<(Config, PathBuf), ConfigError> {
    let path = resolve_path(opt)?;
    let cfg_text = fs::read_to_string(&path).map_err(|e| ConfigError::IoFailed {
        detail: format!("{} ({})", path.display(), e),
    })?;
    let cfg = load(&cfg_text)?;
    validate_config(&cfg)?;
    Ok((cfg, path))
}

#[derive(Clone)]
pub struct InputText {
    pub raw: String,
    pub normalized: String,
}

pub fn read_input(path: &Path) -> Result<(InputText, PathBuf), ConfigError> {
    let bytes = fs::read(path).map_err(|_| ConfigError::NotFound {
        path: path.display().to_string(),
    })?;
    let raw = String::from_utf8(bytes).map_err(|_| ConfigError::InvalidUtf8 {
        path: path.display().to_string(),
        source: None,
    })?;
    let normalized = normalize_lf(strip_bom(raw.clone()));
    Ok((InputText { raw, normalized }, path.to_path_buf()))
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
    if cfg.rule.is_empty() {
        return Err(ConfigError::InvalidValue {
            detail: "no [[rule]] entries configured".to_string(),
        });
    }
    let mut seen = HashSet::new();
    for entry in &cfg.rule {
        if entry.id.trim().is_empty() {
            return Err(ConfigError::InvalidValue {
                detail: "rule id cannot be empty".to_string(),
            });
        }
        if !seen.insert(entry.id.clone()) {
            return Err(ConfigError::InvalidValue {
                detail: format!("duplicate rule id {}", entry.id),
            });
        }
        if !cfg.stages.enabled.contains(&entry.stage) {
            return Err(ConfigError::InvalidValue {
                detail: format!(
                    "rule {} references disabled stage {:?}",
                    entry.id, entry.stage
                ),
            });
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::Stage;

    fn make_rules() -> Vec<RuleConfig> {
        vec![
            RuleConfig {
                id: "a".to_string(),
                script: "a.py".to_string(),
                stage: Stage::RawText,
                enabled: true,
                severity: None,
            },
            RuleConfig {
                id: "b".to_string(),
                script: "b.py".to_string(),
                stage: Stage::RawText,
                enabled: true,
                severity: None,
            },
            RuleConfig {
                id: "c".to_string(),
                script: "c.py".to_string(),
                stage: Stage::RawText,
                enabled: true,
                severity: None,
            },
        ]
    }

    fn is_enabled(rules: &[RuleConfig], id: &str) -> bool {
        rules.iter().find(|r| r.id == id).unwrap().enabled
    }

    #[test]
    fn disable_rules() {
        let mut rules = make_rules();
        let disable = vec!["b".to_string()];
        apply_rule_overrides(&mut rules, &[], &disable).unwrap();
        assert!(is_enabled(&rules, "a"));
        assert!(!is_enabled(&rules, "b"));
        assert!(is_enabled(&rules, "c"));
    }

    #[test]
    fn only_rules() {
        let mut rules = make_rules();
        let only = vec!["c".to_string()];
        apply_rule_overrides(&mut rules, &only, &[]).unwrap();
        assert!(!is_enabled(&rules, "a"));
        assert!(!is_enabled(&rules, "b"));
        assert!(is_enabled(&rules, "c"));
    }

    #[test]
    fn combined_only_and_disable() {
        let mut rules = make_rules();
        let only = vec!["a".to_string(), "b".to_string()];
        let disable = vec!["b".to_string()];
        apply_rule_overrides(&mut rules, &only, &disable).unwrap();
        assert!(is_enabled(&rules, "a"));
        assert!(!is_enabled(&rules, "b"));
        assert!(!is_enabled(&rules, "c"));
    }

    #[test]
    fn invalid_rule_name_errors() {
        let mut rules = make_rules();
        let only = vec!["missing".to_string()];
        let err = apply_rule_overrides(&mut rules, &only, &[]);
        assert!(matches!(err, Err(ConfigError::InvalidValue { .. })));
    }
}
