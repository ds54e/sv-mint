use super::paths::plugin_search_paths;
use super::Config;
use crate::errors::ConfigError;
use crate::types::Severity;
use std::collections::HashSet;
use std::fs;
use std::path::Path;

pub(super) fn validate_config(cfg: &Config) -> Result<(), ConfigError> {
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
        let stage = entry.stage();
        if !cfg.stages.enabled.contains(&stage) {
            return Err(ConfigError::InvalidValue {
                detail: format!("rule {} references disabled stage {:?}", entry.id, stage),
            });
        }
    }
    for stage in &cfg.stages.required {
        if !cfg.stages.enabled.contains(stage) {
            return Err(ConfigError::InvalidValue {
                detail: format!("required stage {:?} must also be enabled", stage),
            });
        }
    }
    if cfg.transport.max_request_bytes == 0 || cfg.transport.max_response_bytes == 0 {
        return Err(ConfigError::InvalidValue {
            detail: "transport byte limits must be greater than zero".to_string(),
        });
    }
    if cfg.transport.warn_margin_bytes > cfg.transport.max_request_bytes {
        return Err(ConfigError::InvalidValue {
            detail: "transport warn_margin_bytes exceeds max_request_bytes".to_string(),
        });
    }
    Ok(())
}

pub(super) fn validate_plugin_dirs(
    cfg: &Config,
    fallback_root: &Path,
    has_user_roots: bool,
    needs_default_root: bool,
) -> Result<(), ConfigError> {
    if let Some(root) = cfg.plugin.normalized_root.as_ref() {
        validate_dir(root, "plugin.root")?;
    }
    for search in &cfg.plugin.normalized_search_paths {
        validate_dir(search, "plugin.search_paths entry")?;
    }
    if has_user_roots || !needs_default_root {
        return Ok(());
    }
    validate_dir(fallback_root, "default plugin directory")
}

fn validate_dir(path: &Path, label: &str) -> Result<(), ConfigError> {
    let meta = fs::metadata(path).map_err(|_| ConfigError::InvalidValue {
        detail: format!("{label} not found: {}", path.display()),
    })?;
    if !meta.is_dir() {
        return Err(ConfigError::InvalidValue {
            detail: format!("{label} is not a directory: {}", path.display()),
        });
    }
    Ok(())
}

pub(super) fn validate_rule_script_paths(cfg: &Config) -> Result<(), ConfigError> {
    for rule in &cfg.rule {
        let script = rule.script.trim();
        if script.is_empty() {
            continue;
        }
        if let Some(sev) = &rule.severity {
            if severity_from_str(sev).is_none() {
                return Err(ConfigError::InvalidValue {
                    detail: format!("rule {} severity must be error|warning|info", rule.id),
                });
            }
        }
        let path = Path::new(script);
        let mut probed = if path.is_absolute() {
            vec![path.to_path_buf()]
        } else {
            plugin_search_paths(cfg, script)
        };
        if probed.iter().any(|p| p.exists()) {
            continue;
        }
        probed.sort();
        probed.dedup();
        let joined = probed
            .iter()
            .map(|p| p.display().to_string())
            .collect::<Vec<_>>()
            .join(", ");
        return Err(ConfigError::InvalidValue {
            detail: format!("rule {} script not found; searched {}", rule.id, joined),
        });
    }
    Ok(())
}

fn severity_from_str(s: &str) -> Option<Severity> {
    match s {
        "error" => Some(Severity::Error),
        "warning" => Some(Severity::Warning),
        "info" => Some(Severity::Info),
        _ => None,
    }
}
