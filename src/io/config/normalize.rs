use super::validate::validate_plugin_dirs;
use super::{Config, RuleConfig};
use crate::errors::ConfigError;
use crate::types::Stage;
use std::path::{Path, PathBuf};

pub(super) fn normalize_rule_scripts(cfg: &mut Config, base_dir: &Path) -> Result<(), ConfigError> {
    cfg.plugin.config_dir = Some(base_dir.to_path_buf());
    cfg.plugin.normalized_root = cfg.plugin.root.as_ref().map(|root| to_abs(base_dir, root));
    cfg.plugin.normalized_search_paths = cfg.plugin.search_paths.iter().map(|p| to_abs(base_dir, p)).collect();
    let mut search_roots = Vec::new();
    if let Some(root) = cfg.plugin.normalized_root.clone() {
        search_roots.push(root);
    }
    if !cfg.plugin.normalized_search_paths.is_empty() {
        search_roots.extend(cfg.plugin.normalized_search_paths.clone());
    }
    let has_user_roots = !search_roots.is_empty();
    let fallback_root = base_dir.join("plugins");
    let needs_default_root = cfg.rule.iter().any(|r| r.script.trim().is_empty());
    validate_plugin_dirs(cfg, &fallback_root, has_user_roots, needs_default_root)?;
    if !has_user_roots {
        search_roots.push(fallback_root.clone());
    }
    for entry in &mut cfg.rule {
        if entry.script.trim().is_empty() {
            entry.script = derive_script_from_id(entry, &search_roots)?;
            if !has_user_roots {
                let absolute = fallback_root.join(entry.script.as_str());
                entry.script = absolute.to_string_lossy().into_owned();
            }
        }
        if has_user_roots {
            continue;
        }
        let script_path = Path::new(&entry.script);
        if script_path.is_absolute() {
            continue;
        }
        let candidate = base_dir.join(script_path);
        entry.script = candidate.to_string_lossy().into_owned();
    }
    Ok(())
}

pub(super) fn derive_script_from_id(entry: &RuleConfig, roots: &[PathBuf]) -> Result<String, ConfigError> {
    if roots.is_empty() {
        return Err(ConfigError::InvalidValue {
            detail: format!(
                "rule {} missing script; set plugin.root/search_paths or specify script explicitly",
                entry.id
            ),
        });
    }
    let mut found = Vec::new();
    for root in roots {
        for stage in ["raw", "pp", "cst", "ast"] {
            let file = format!("{}.{}.py", entry.id, stage);
            let path = root.join(&file);
            if path.exists() {
                found.push(file);
            }
        }
    }
    found.sort();
    found.dedup();
    match found.len() {
        1 => Ok(found.remove(0)),
        0 => Err(ConfigError::InvalidValue {
            detail: format!(
                "rule {} missing script and no bundled file named {}.{{raw,pp,cst,ast}}.py exists under plugin roots",
                entry.id, entry.id
            ),
        }),
        _ => Err(ConfigError::InvalidValue {
            detail: format!(
                "rule {} matches multiple scripts {:?}; specify script explicitly",
                entry.id, found
            ),
        }),
    }
}

pub(super) fn infer_rule_stages(rules: &mut [RuleConfig]) -> Result<(), ConfigError> {
    for rule in rules {
        if rule.stage.is_some() {
            continue;
        }
        let inferred = infer_stage_from_script(rule)?;
        rule.stage = Some(inferred);
    }
    Ok(())
}

pub(super) fn infer_stage_from_script(rule: &RuleConfig) -> Result<Stage, ConfigError> {
    let path = Path::new(&rule.script);
    let file_name = path
        .file_name()
        .and_then(|s| s.to_str())
        .ok_or_else(|| ConfigError::InvalidValue {
            detail: format!("rule {} missing stage and script {} is invalid", rule.id, rule.script),
        })?;
    let ext = path.extension().and_then(|s| s.to_str()).unwrap_or("");
    if ext != "py" {
        return Err(ConfigError::InvalidValue {
            detail: format!(
                "rule {} missing stage and script {} must end with .py",
                rule.id, file_name
            ),
        });
    }
    let stem = path
        .file_stem()
        .and_then(|s| s.to_str())
        .ok_or_else(|| ConfigError::InvalidValue {
            detail: format!("rule {} missing stage and script {} is invalid", rule.id, rule.script),
        })?;
    let suffix = stem
        .rsplit_once('.')
        .map(|(_, suffix)| suffix)
        .ok_or_else(|| ConfigError::InvalidValue {
            detail: format!(
                "rule {} missing stage and script {} lacks .<stage> suffix",
                rule.id, file_name
            ),
        })?;
    match suffix {
        "raw" => Ok(Stage::RawText),
        "pp" => Ok(Stage::PpText),
        "cst" => Ok(Stage::Cst),
        "ast" => Ok(Stage::Ast),
        other => Err(ConfigError::InvalidValue {
            detail: format!(
                "rule {} missing stage and script {} has unsupported stage suffix {}",
                rule.id, file_name, other
            ),
        }),
    }
}

pub(super) fn to_abs(base: &Path, rel: &str) -> PathBuf {
    let p = Path::new(rel);
    if p.is_absolute() {
        p.to_path_buf()
    } else {
        base.join(p)
    }
}
