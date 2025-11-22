use super::normalize::{infer_rule_stages, normalize_rule_scripts};
use super::validate::{validate_config, validate_rule_script_paths};
use super::Config;
use crate::errors::ConfigError;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

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
    let path_rel = resolve_path(opt)?;
    let cwd = env::current_dir().map_err(|e| ConfigError::IoFailed {
        detail: format!("{} ({})", path_rel.display(), e),
    })?;
    let path = if path_rel.is_absolute() {
        path_rel
    } else {
        cwd.join(path_rel)
    };
    let cfg_text = fs::read_to_string(&path).map_err(|e| ConfigError::IoFailed {
        detail: format!("{} ({})", path.display(), e),
    })?;
    let mut cfg = load(&cfg_text)?;
    let base_dir = path
        .parent()
        .map(Path::to_path_buf)
        .unwrap_or_else(|| PathBuf::from("."));
    warn_on_unknown_keys(&cfg_text);
    normalize_rule_scripts(&mut cfg, &base_dir)?;
    infer_rule_stages(&mut cfg.rule)?;
    validate_config(&cfg)?;
    validate_rule_script_paths(&cfg)?;
    Ok((cfg, path))
}

pub(super) fn warn_on_unknown_keys(cfg_text: &str) {
    if let Ok(table) = cfg_text.parse::<toml::Value>() {
        if let Some(obj) = table.as_table() {
            for (k, v) in obj {
                match k.as_str() {
                    "logging" | "defaults" | "plugin" | "stages" | "svparser" | "transport" => {
                        warn_nested_unknowns(k, v);
                    }
                    "rule" => warn_rule_unknowns(v),
                    other => tracing::warn!("unknown top-level key: {}", other),
                }
            }
        }
    }
}

fn warn_nested_unknowns(parent: &str, val: &toml::Value) {
    let Some(table) = val.as_table() else { return };
    let known: &'static [&'static str] = match parent {
        "logging" => &[
            "level",
            "stderr_snippet_bytes",
            "show_stage_events",
            "show_plugin_events",
            "show_parse_events",
            "format",
        ],
        "defaults" => &["timeout_ms_per_file"],
        "plugin" => &["cmd", "args", "root", "search_paths"],
        "stages" => &["enabled", "required"],
        "svparser" => &[
            "include_paths",
            "defines",
            "strip_comments",
            "ignore_include",
            "allow_incomplete",
        ],
        "transport" => &[
            "max_request_bytes",
            "warn_margin_bytes",
            "max_response_bytes",
            "on_exceed",
            "fail_ci_on_skip",
        ],
        _ => &[],
    };
    for key in table.keys() {
        if !known.contains(&key.as_str()) {
            tracing::warn!("unknown key {}.{}", parent, key);
        }
    }
}

fn warn_rule_unknowns(val: &toml::Value) {
    let Some(array) = val.as_array() else { return };
    for (idx, entry) in array.iter().enumerate() {
        let Some(table) = entry.as_table() else { continue };
        let known = ["id", "script", "stage", "enabled", "severity"];
        for key in table.keys() {
            if !known.contains(&key.as_str()) {
                tracing::warn!("unknown key rule[{}].{}", idx, key);
            }
        }
    }
}
