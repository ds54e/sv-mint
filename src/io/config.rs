use crate::errors::ConfigError;
use crate::svparser::SvParserCfg;
use crate::textutil::{normalize_lf, strip_bom};
use crate::types::Severity;
use crate::types::Stage;
use serde::Deserialize;
use std::collections::{HashMap, HashSet};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use toml::Value as TomlValue;

#[derive(serde::Deserialize, Clone)]
pub struct LoggingConfig {
    #[serde(default = "default_log_level")]
    pub level: String,
    #[serde(default = "default_stderr_snippet_bytes")]
    pub stderr_snippet_bytes: usize,
    #[serde(default = "default_false")]
    pub show_stage_events: bool,
    #[serde(default = "default_false")]
    pub show_plugin_events: bool,
    #[serde(default = "default_false")]
    pub show_parse_events: bool,
    #[serde(default)]
    pub format: LogFormat,
    #[serde(flatten, default)]
    pub extra: HashMap<String, TomlValue>,
}

impl Default for LoggingConfig {
    fn default() -> Self {
        Self {
            level: default_log_level(),
            stderr_snippet_bytes: default_stderr_snippet_bytes(),
            show_stage_events: default_false(),
            show_plugin_events: default_false(),
            show_parse_events: default_false(),
            format: LogFormat::Text,
            extra: HashMap::new(),
        }
    }
}

#[derive(Deserialize, Clone, Copy, Default)]
#[serde(rename_all = "lowercase")]
pub enum LogFormat {
    #[default]
    Text,
    Json,
}

#[derive(Deserialize, Clone, Copy, Default)]
#[serde(rename_all = "lowercase")]
pub enum TransportOnExceed {
    #[default]
    Skip,
    Error,
}

fn default_warn_margin_bytes() -> usize {
    1_048_576
}

#[derive(Deserialize, Clone)]
pub struct TransportConfig {
    #[serde(default = "default_max_request_bytes")]
    pub max_request_bytes: usize,
    #[serde(default = "default_warn_margin_bytes")]
    pub warn_margin_bytes: usize,
    #[serde(default = "default_max_request_bytes")]
    pub max_response_bytes: usize,
    #[serde(default)]
    pub on_exceed: TransportOnExceed,
    #[serde(default)]
    pub fail_ci_on_skip: bool,
}

impl Default for TransportConfig {
    fn default() -> Self {
        Self {
            max_request_bytes: default_max_request_bytes(),
            warn_margin_bytes: default_warn_margin_bytes(),
            max_response_bytes: default_max_request_bytes(),
            on_exceed: TransportOnExceed::Skip,
            fail_ci_on_skip: false,
        }
    }
}

#[derive(Deserialize)]
pub struct Config {
    #[serde(default)]
    pub logging: LoggingConfig,
    #[serde(default)]
    pub defaults: Defaults,
    #[serde(default)]
    pub plugin: Plugin,
    #[serde(default)]
    pub stages: Stages,
    #[serde(default)]
    pub svparser: SvParserCfg,
    #[serde(default)]
    pub rule: Vec<RuleConfig>,
    #[serde(default)]
    pub transport: TransportConfig,
}

#[derive(Deserialize)]
pub struct Defaults {
    #[serde(default = "default_timeout_ms")]
    pub timeout_ms_per_file: u64,
}

impl Default for Defaults {
    fn default() -> Self {
        Self {
            timeout_ms_per_file: default_timeout_ms(),
        }
    }
}

#[derive(Deserialize)]
pub struct Plugin {
    #[serde(default = "default_plugin_cmd")]
    pub cmd: String,
    #[serde(default = "default_plugin_args")]
    pub args: Vec<String>,
    #[serde(default)]
    pub root: Option<String>,
    #[serde(default)]
    pub search_paths: Vec<String>,
    #[serde(skip)]
    pub normalized_root: Option<PathBuf>,
    #[serde(skip)]
    pub normalized_search_paths: Vec<PathBuf>,
    #[serde(skip)]
    pub config_dir: Option<PathBuf>,
}

impl Default for Plugin {
    fn default() -> Self {
        Self {
            cmd: default_plugin_cmd(),
            args: default_plugin_args(),
            root: None,
            search_paths: Vec::new(),
            normalized_root: None,
            normalized_search_paths: Vec::new(),
            config_dir: None,
        }
    }
}

#[derive(Deserialize)]
pub struct Stages {
    #[serde(default = "default_enabled_stages")]
    pub enabled: Vec<crate::types::Stage>,
    #[serde(default = "default_required_stages")]
    pub required: Vec<Stage>,
}

impl Default for Stages {
    fn default() -> Self {
        Self {
            enabled: default_enabled_stages(),
            required: default_required_stages(),
        }
    }
}

#[derive(Deserialize, Clone)]
pub struct RuleConfig {
    pub id: String,
    #[serde(default)]
    pub script: String,
    #[serde(default)]
    pub stage: Option<Stage>,
    #[serde(default = "default_true")]
    pub enabled: bool,
    #[serde(default)]
    pub severity: Option<String>,
}

impl RuleConfig {
    pub fn stage(&self) -> Stage {
        self.stage.expect("rule stage must be set during config load")
    }
}

fn default_true() -> bool {
    true
}

fn default_timeout_ms() -> u64 {
    6000
}

fn default_max_request_bytes() -> usize {
    16_777_216
}

fn default_log_level() -> String {
    "info".to_string()
}

fn default_stderr_snippet_bytes() -> usize {
    2048
}

fn default_false() -> bool {
    false
}

fn default_plugin_cmd() -> String {
    "python3".to_string()
}

fn default_plugin_args() -> Vec<String> {
    vec!["-u".to_string(), "-B".to_string()]
}

fn default_enabled_stages() -> Vec<Stage> {
    vec![Stage::RawText, Stage::PpText, Stage::Cst, Stage::Ast]
}

fn default_required_stages() -> Vec<Stage> {
    vec![Stage::RawText, Stage::PpText]
}

pub fn apply_rule_overrides(rules: &mut [RuleConfig], only: &[String], disable: &[String]) -> Result<(), ConfigError> {
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

fn warn_on_unknown_keys(cfg_text: &str) {
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

fn normalize_rule_scripts(cfg: &mut Config, base_dir: &Path) -> Result<(), ConfigError> {
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

fn derive_script_from_id(entry: &RuleConfig, roots: &[PathBuf]) -> Result<String, ConfigError> {
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

fn infer_rule_stages(rules: &mut [RuleConfig]) -> Result<(), ConfigError> {
    for rule in rules {
        if rule.stage.is_some() {
            continue;
        }
        let inferred = infer_stage_from_script(rule)?;
        rule.stage = Some(inferred);
    }
    Ok(())
}

fn infer_stage_from_script(rule: &RuleConfig) -> Result<Stage, ConfigError> {
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

fn to_abs(base: &Path, rel: &str) -> PathBuf {
    let p = Path::new(rel);
    if p.is_absolute() {
        p.to_path_buf()
    } else {
        base.join(p)
    }
}

fn validate_plugin_dirs(
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

fn push_unique(out: &mut Vec<PathBuf>, seen: &mut HashSet<PathBuf>, p: PathBuf) {
    if seen.insert(p.clone()) {
        out.push(p);
    }
}

pub fn plugin_search_paths(cfg: &Config, rel: &str) -> Vec<PathBuf> {
    let mut out = Vec::new();
    let mut seen = HashSet::new();
    let has_user_roots = cfg.plugin.normalized_root.is_some() || !cfg.plugin.normalized_search_paths.is_empty();
    if let Some(root) = cfg.plugin.normalized_root.as_ref() {
        push_unique(&mut out, &mut seen, root.join(rel));
    }
    for extra in &cfg.plugin.normalized_search_paths {
        push_unique(&mut out, &mut seen, extra.join(rel));
    }
    if !has_user_roots && cfg.plugin.config_dir.is_some() {
        let config_dir = cfg.plugin.config_dir.as_ref().unwrap();
        push_unique(&mut out, &mut seen, config_dir.join("plugins").join(rel));
    }
    if !out.is_empty() {
        return out;
    }
    if let Ok(cwd) = env::current_dir() {
        push_unique(&mut out, &mut seen, cwd.join(rel));
    }
    if let Ok(exe) = env::current_exe() {
        if let Some(base) = exe.parent() {
            push_unique(&mut out, &mut seen, base.join(rel));
        }
    }
    out
}

fn validate_rule_script_paths(cfg: &Config) -> Result<(), ConfigError> {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::Stage;
    use std::fs;
    use std::path::Path;
    use tempfile::tempdir;

    fn make_rules() -> Vec<RuleConfig> {
        vec![
            RuleConfig {
                id: "a".to_string(),
                script: "a.py".to_string(),
                stage: Some(Stage::RawText),
                enabled: true,
                severity: None,
            },
            RuleConfig {
                id: "b".to_string(),
                script: "b.py".to_string(),
                stage: Some(Stage::RawText),
                enabled: true,
                severity: None,
            },
            RuleConfig {
                id: "c".to_string(),
                script: "c.py".to_string(),
                stage: Some(Stage::RawText),
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

    #[test]
    fn infers_stage_from_script_suffix() {
        let mut rules = vec![RuleConfig {
            id: "flow.wait_macro_required".to_string(),
            script: "plugins/flow.wait_macro_required.raw.py".to_string(),
            stage: None,
            enabled: true,
            severity: None,
        }];
        infer_rule_stages(&mut rules).unwrap();
        assert!(matches!(rules[0].stage(), Stage::RawText));
    }

    #[test]
    fn errors_when_stage_suffix_missing() {
        let mut rules = vec![RuleConfig {
            id: "example".to_string(),
            script: "plugins/example.py".to_string(),
            stage: None,
            enabled: true,
            severity: None,
        }];
        let err = infer_rule_stages(&mut rules);
        assert!(matches!(err, Err(ConfigError::InvalidValue { .. })));
    }

    #[test]
    fn config_sections_use_defaults() {
        let cfg = load(
            r#"
[[rule]]
id = "dv.dpi.import_prefix"
script = "dv.dpi.import_prefix.raw.py"
stage = "raw_text"
"#,
        )
        .expect("load defaults");
        assert_eq!(cfg.defaults.timeout_ms_per_file, 6000);
        assert_eq!(cfg.plugin.cmd, "python3");
        assert_eq!(cfg.plugin.args, vec!["-u", "-B"]);
        assert_eq!(
            cfg.stages.enabled,
            vec![Stage::RawText, Stage::PpText, Stage::Cst, Stage::Ast]
        );
        assert_eq!(cfg.stages.required, vec![Stage::RawText, Stage::PpText]);
    }

    #[test]
    fn derives_scripts_without_plugin_root() {
        let tmp_dir = tempdir().expect("tempdir");
        let plugins = tmp_dir.path().join("plugins");
        fs::create_dir_all(&plugins).expect("plugins dir");
        let script_path = plugins.join("dv.dpi.import_prefix.raw.py");
        fs::write(&script_path, "print('ok')").expect("script file");
        let mut cfg = load(
            r#"
[[rule]]
id = "dv.dpi.import_prefix"
"#,
        )
        .expect("load rule");
        normalize_rule_scripts(&mut cfg, tmp_dir.path()).expect("normalize");
        assert_eq!(cfg.rule.len(), 1);
        let script = &cfg.rule[0].script;
        assert!(Path::new(script).is_absolute());
        assert!(script.ends_with("dv.dpi.import_prefix.raw.py"));
    }

    #[test]
    fn errors_when_search_path_missing() {
        let tmp_dir = tempdir().expect("tempdir");
        let mut cfg = load(
            r#"
[plugin]
search_paths = ["./plugins-extra"]

[[rule]]
id = "dv.dpi.import_prefix"
script = "dv.dpi.import_prefix.raw.py"
stage = "raw_text"
"#,
        )
        .expect("load rule");
        let err = normalize_rule_scripts(&mut cfg, tmp_dir.path()).unwrap_err();
        match err {
            ConfigError::InvalidValue { detail } => {
                assert!(detail.contains("plugin.search_paths"));
                assert!(detail.contains("plugins-extra"));
            }
            other => panic!("unexpected error {other:?}"),
        }
    }

    #[test]
    fn errors_when_default_plugins_dir_missing() {
        let tmp_dir = tempdir().expect("tempdir");
        let mut cfg = load(
            r#"
[[rule]]
id = "dv.dpi.import_prefix"
"#,
        )
        .expect("load rule");
        let err = normalize_rule_scripts(&mut cfg, tmp_dir.path()).unwrap_err();
        match err {
            ConfigError::InvalidValue { detail } => {
                assert!(detail.contains("default plugin directory"));
                assert!(detail.contains(&tmp_dir.path().join("plugins").display().to_string()));
            }
            other => panic!("unexpected error {other:?}"),
        }
    }

    #[test]
    fn resolves_default_plugins_relative_to_config_dir() {
        let tmp_dir = tempdir().expect("tempdir");
        let plugins = tmp_dir.path().join("plugins");
        fs::create_dir_all(&plugins).expect("plugins dir");
        let script_path = plugins.join("dv.dpi.import_prefix.raw.py");
        fs::write(&script_path, "print('ok')").expect("script file");
        let mut cfg = load(
            r#"
[[rule]]
id = "dv.dpi.import_prefix"
"#,
        )
        .expect("load rule");
        normalize_rule_scripts(&mut cfg, tmp_dir.path()).expect("normalize");
        infer_rule_stages(&mut cfg.rule).expect("stage");
        let resolved = super::plugin_search_paths(&cfg, "dv.dpi.import_prefix.raw.py");
        assert!(resolved.iter().any(|p| p == &script_path));
        let err = validate_rule_script_paths(&cfg);
        assert!(err.is_ok());
    }

    #[test]
    fn errors_when_rule_script_missing_lists_search_paths() {
        let tmp_dir = tempdir().expect("tempdir");
        fs::create_dir_all(tmp_dir.path().join("plugins")).expect("plugins dir");
        let mut cfg = load(
            r#"
[[rule]]
id = "dv.dpi.import_prefix"
script = "dv.dpi.import_prefix.raw.py"
stage = "raw_text"
"#,
        )
        .expect("load rule");
        normalize_rule_scripts(&mut cfg, tmp_dir.path()).expect("normalize");
        infer_rule_stages(&mut cfg.rule).expect("stage");
        let err = validate_rule_script_paths(&cfg).unwrap_err();
        match err {
            ConfigError::InvalidValue { detail } => {
                assert!(detail.contains("dv.dpi.import_prefix"));
                assert!(detail.contains("searched"));
                let expected = tmp_dir.path().join("dv.dpi.import_prefix.raw.py");
                assert!(detail.contains(&expected.display().to_string()));
            }
            other => panic!("unexpected error {other:?}"),
        }
    }

    #[test]
    fn resolves_script_under_relative_plugin_root() {
        let tmp_dir = tempdir().expect("tempdir");
        let root = tmp_dir.path().join("plugins-root");
        fs::create_dir_all(&root).expect("root dir");
        let script_path = root.join("dv.dpi.import_prefix.raw.py");
        fs::write(&script_path, "print('ok')").expect("script file");
        let mut cfg = load(
            r#"
[plugin]
root = "./plugins-root"

[[rule]]
id = "dv.dpi.import_prefix"
"#,
        )
        .expect("load rule");
        normalize_rule_scripts(&mut cfg, tmp_dir.path()).expect("normalize");
        infer_rule_stages(&mut cfg.rule).expect("stage");
        let paths = plugin_search_paths(&cfg, "dv.dpi.import_prefix.raw.py");
        assert_eq!(paths[0], root.join("dv.dpi.import_prefix.raw.py"));
        validate_rule_script_paths(&cfg).expect("scripts exist");
    }

    #[test]
    fn resolves_script_from_search_paths_and_root_prefers_root() {
        let tmp_dir = tempdir().expect("tempdir");
        let root = tmp_dir.path().join("plugins-root");
        let extra = tmp_dir.path().join("plugins-extra");
        fs::create_dir_all(&root).expect("root dir");
        fs::create_dir_all(&extra).expect("extra dir");
        let in_root = root.join("dv.dpi.import_prefix.raw.py");
        let in_extra = extra.join("dv.dpi.import_prefix.raw.py");
        fs::write(&in_root, "print('root')").expect("script file");
        fs::write(&in_extra, "print('extra')").expect("script file");
        let mut cfg = load(
            r#"
[plugin]
root = "./plugins-root"
search_paths = ["./plugins-extra"]

[[rule]]
id = "dv.dpi.import_prefix"
script = "dv.dpi.import_prefix.raw.py"
stage = "raw_text"
"#,
        )
        .expect("load rule");
        normalize_rule_scripts(&mut cfg, tmp_dir.path()).expect("normalize");
        infer_rule_stages(&mut cfg.rule).expect("stage");
        let paths = plugin_search_paths(&cfg, &cfg.rule[0].script);
        assert_eq!(paths[0], in_root);
        validate_rule_script_paths(&cfg).expect("scripts exist");
    }

    #[test]
    fn errors_for_missing_absolute_script_with_probes_listed() {
        let missing = PathBuf::from("/tmp/sv-mint-test-missing.py");
        if missing.exists() {
            fs::remove_file(&missing).unwrap();
        }
        let mut cfg = load(&format!(
            r#"
[[rule]]
id = "dv.dpi.import_prefix"
script = "{}"
stage = "raw_text"
"#,
            missing.display()
        ))
        .expect("load rule");
        let tmp_dir = tempdir().expect("tempdir");
        normalize_rule_scripts(&mut cfg, tmp_dir.path()).expect("normalize");
        infer_rule_stages(&mut cfg.rule).expect("stage");
        let err = validate_rule_script_paths(&cfg).unwrap_err();
        match err {
            ConfigError::InvalidValue { detail } => {
                assert!(detail.contains("dv.dpi.import_prefix"));
                assert!(detail.contains(&missing.display().to_string()));
            }
            other => panic!("unexpected error {other:?}"),
        }
    }

    #[test]
    fn search_paths_and_root_dedup_probes() {
        let tmp_dir = tempdir().expect("tempdir");
        let root = tmp_dir.path().join("plugins-root");
        fs::create_dir_all(&root).expect("root dir");
        let mut cfg = load(
            r#"
[plugin]
root = "./plugins-root"
search_paths = ["./plugins-root"]

[[rule]]
id = "dv.dpi.import_prefix"
script = "dv.dpi.import_prefix.raw.py"
stage = "raw_text"
"#,
        )
        .expect("load rule");
        normalize_rule_scripts(&mut cfg, tmp_dir.path()).expect("normalize");
        infer_rule_stages(&mut cfg.rule).expect("stage");
        let paths = plugin_search_paths(&cfg, &cfg.rule[0].script);
        assert_eq!(paths[0], root.join("dv.dpi.import_prefix.raw.py"));
        assert_eq!(paths.len(), 1);
    }

    #[test]
    fn warns_on_unknown_top_level_and_nested_keys() {
        let cfg_text = r#"
[logging]
stderr_snippet_bytes = 10
show_stage_events = true
show_plugin_events = true
show_parse_events = true
level = "debug"
unknown_log = true

[plugin]
cmd = "python3"
extra_plugin = 1

[transport]
max_request_bytes = 10
max_response_bytes = 10
unknown_transport = true

[mystery]
foo = "bar"
"#;
        let _ = load(cfg_text).expect("load");
        warn_on_unknown_keys(cfg_text);
    }

    #[test]
    fn warns_on_unknown_rule_keys() {
        let cfg_text = r#"
[[rule]]
id = "dv.dpi.import_prefix"
script = "dv.dpi.import_prefix.raw.py"
stage = "raw_text"
unknown_field = true
"#;
        let cfg = load(cfg_text).expect("load");
        assert_eq!(cfg.rule.len(), 1);
        warn_on_unknown_keys(cfg_text);
    }

    #[test]
    fn errors_on_invalid_severity_override() {
        let tmp_dir = tempdir().expect("tempdir");
        fs::create_dir_all(tmp_dir.path().join("plugins")).expect("plugins dir");
        let mut cfg = load(
            r#"
[[rule]]
id = "dv.dpi.import_prefix"
script = "dv.dpi.import_prefix.raw.py"
severity = "fatal"
stage = "raw_text"
"#,
        )
        .expect("load rule");
        normalize_rule_scripts(&mut cfg, tmp_dir.path()).expect("normalize");
        infer_rule_stages(&mut cfg.rule).expect("stage");
        let err = validate_rule_script_paths(&cfg).unwrap_err();
        match err {
            ConfigError::InvalidValue { detail } => {
                assert!(detail.contains("dv.dpi.import_prefix"));
                assert!(detail.contains("severity"));
            }
            other => panic!("unexpected error {other:?}"),
        }
    }

    #[test]
    fn logging_partial_overrides_apply_defaults() {
        let cfg = load(
            r#"
[logging]
stderr_snippet_bytes = 512
"#,
        )
        .expect("load");
        assert_eq!(cfg.logging.stderr_snippet_bytes, 512);
        assert!(!cfg.logging.show_stage_events);
        assert!(!cfg.logging.show_plugin_events);
        assert!(!cfg.logging.show_parse_events);
        assert_eq!(cfg.logging.level, "info");
    }

    #[test]
    fn plugin_args_can_be_empty() {
        let cfg = load(
            r#"
[plugin]
args = []

[[rule]]
id = "dv.dpi.import_prefix"
script = "dv.dpi.import_prefix.raw.py"
stage = "raw_text"
"#,
        )
        .expect("load");
        assert!(cfg.plugin.args.is_empty());
    }

    #[test]
    fn stages_enabled_empty_errors() {
        let cfg = load(
            r#"
[stages]
enabled = []

[[rule]]
id = "dv.dpi.import_prefix"
script = "dv.dpi.import_prefix.raw.py"
stage = "raw_text"
"#,
        )
        .expect("load");
        let err = validate_config(&cfg).unwrap_err();
        assert!(matches!(err, ConfigError::InvalidValue { .. }));
    }

    #[test]
    fn svparser_defaults_fill_in() {
        let cfg = load(
            r#"
[[rule]]
id = "dv.dpi.import_prefix"
script = "dv.dpi.import_prefix.raw.py"
stage = "raw_text"
"#,
        )
        .expect("load");
        assert!(cfg.svparser.include_paths.is_empty());
        assert!(cfg.svparser.defines.is_empty());
        assert!(cfg.svparser.strip_comments);
        assert!(cfg.svparser.ignore_include);
        assert!(cfg.svparser.allow_incomplete);
    }

    #[test]
    fn transport_invalid_values_error() {
        let cfg = load(
            r#"
[transport]
max_request_bytes = 0

[[rule]]
id = "dv.dpi.import_prefix"
script = "dv.dpi.import_prefix.raw.py"
stage = "raw_text"
"#,
        )
        .expect("load");
        let err = validate_config(&cfg).unwrap_err();
        assert!(matches!(err, ConfigError::InvalidValue { .. }));

        let cfg = load(
            r#"
[transport]
max_request_bytes = 100
warn_margin_bytes = 200
max_response_bytes = 100

[[rule]]
id = "dv.dpi.import_prefix"
script = "dv.dpi.import_prefix.raw.py"
stage = "raw_text"
"#,
        )
        .expect("load");
        let err = validate_config(&cfg).unwrap_err();
        assert!(matches!(err, ConfigError::InvalidValue { .. }));
    }

    #[test]
    fn duplicate_rule_ids_error() {
        let cfg = load(
            r#"
[[rule]]
id = "dv.dpi.import_prefix"
script = "a.raw.py"
stage = "raw_text"

[[rule]]
id = "dv.dpi.import_prefix"
script = "b.raw.py"
stage = "raw_text"
"#,
        )
        .expect("load");
        let err = validate_config(&cfg).unwrap_err();
        assert!(matches!(err, ConfigError::InvalidValue { .. }));
    }

    #[test]
    fn plugin_root_must_be_directory() {
        let tmp = tempdir().unwrap();
        let file_path = tmp.path().join("not_a_dir");
        fs::write(&file_path, "x").unwrap();
        let path_str = file_path.to_string_lossy().replace('\\', "\\\\");
        let mut cfg = load(&format!(
            r#"
[plugin]
root = "{}"

[[rule]]
id = "dv.dpi.import_prefix"
"#,
            path_str
        ))
        .expect("load");
        let err = normalize_rule_scripts(&mut cfg, tmp.path()).unwrap_err();
        assert!(matches!(err, ConfigError::InvalidValue { .. }));
    }
}
