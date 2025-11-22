use super::loader::warn_on_unknown_keys;
use super::normalize::{infer_rule_stages, normalize_rule_scripts};
use super::validate::{validate_config, validate_rule_script_paths};
use super::*;
use crate::errors::ConfigError;
use crate::types::Stage;
use std::fs;
use std::path::{Path, PathBuf};
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
id = "dv_dpi_import_prefix"
script = "dv_dpi_import_prefix.raw.py"
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
    let script_path = plugins.join("dv_dpi_import_prefix.raw.py");
    fs::write(&script_path, "print('ok')").expect("script file");
    let mut cfg = load(
        r#"
[[rule]]
id = "dv_dpi_import_prefix"
"#,
    )
    .expect("load rule");
    normalize_rule_scripts(&mut cfg, tmp_dir.path()).expect("normalize");
    assert_eq!(cfg.rule.len(), 1);
    let script = &cfg.rule[0].script;
    assert!(Path::new(script).is_absolute());
    assert!(script.ends_with("dv_dpi_import_prefix.raw.py"));
}

#[test]
fn errors_when_search_path_missing() {
    let tmp_dir = tempdir().expect("tempdir");
    let mut cfg = load(
        r#"
[plugin]
search_paths = ["./plugins-extra"]

[[rule]]
id = "dv_dpi_import_prefix"
script = "dv_dpi_import_prefix.raw.py"
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
id = "dv_dpi_import_prefix"
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
    let script_path = plugins.join("dv_dpi_import_prefix.raw.py");
    fs::write(&script_path, "print('ok')").expect("script file");
    let mut cfg = load(
        r#"
[[rule]]
id = "dv_dpi_import_prefix"
"#,
    )
    .expect("load rule");
    normalize_rule_scripts(&mut cfg, tmp_dir.path()).expect("normalize");
    infer_rule_stages(&mut cfg.rule).expect("stage");
    let resolved = super::plugin_search_paths(&cfg, "dv_dpi_import_prefix.raw.py");
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
id = "dv_dpi_import_prefix"
script = "dv_dpi_import_prefix.raw.py"
stage = "raw_text"
"#,
    )
    .expect("load rule");
    normalize_rule_scripts(&mut cfg, tmp_dir.path()).expect("normalize");
    infer_rule_stages(&mut cfg.rule).expect("stage");
    let err = validate_rule_script_paths(&cfg).unwrap_err();
    match err {
        ConfigError::InvalidValue { detail } => {
            assert!(detail.contains("dv_dpi_import_prefix"));
            assert!(detail.contains("searched"));
            let expected = tmp_dir.path().join("dv_dpi_import_prefix.raw.py");
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
    let script_path = root.join("dv_dpi_import_prefix.raw.py");
    fs::write(&script_path, "print('ok')").expect("script file");
    let mut cfg = load(
        r#"
[plugin]
root = "./plugins-root"

[[rule]]
id = "dv_dpi_import_prefix"
"#,
    )
    .expect("load rule");
    normalize_rule_scripts(&mut cfg, tmp_dir.path()).expect("normalize");
    infer_rule_stages(&mut cfg.rule).expect("stage");
    let paths = plugin_search_paths(&cfg, "dv_dpi_import_prefix.raw.py");
    assert_eq!(paths[0], root.join("dv_dpi_import_prefix.raw.py"));
    validate_rule_script_paths(&cfg).expect("scripts exist");
}

#[test]
fn resolves_script_from_search_paths_and_root_prefers_root() {
    let tmp_dir = tempdir().expect("tempdir");
    let root = tmp_dir.path().join("plugins-root");
    let extra = tmp_dir.path().join("plugins-extra");
    fs::create_dir_all(&root).expect("root dir");
    fs::create_dir_all(&extra).expect("extra dir");
    let in_root = root.join("dv_dpi_import_prefix.raw.py");
    let in_extra = extra.join("dv_dpi_import_prefix.raw.py");
    fs::write(&in_root, "print('root')").expect("script file");
    fs::write(&in_extra, "print('extra')").expect("script file");
    let mut cfg = load(
        r#"
[plugin]
root = "./plugins-root"
search_paths = ["./plugins-extra"]

[[rule]]
id = "dv_dpi_import_prefix"
script = "dv_dpi_import_prefix.raw.py"
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
id = "dv_dpi_import_prefix"
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
            assert!(detail.contains("dv_dpi_import_prefix"));
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
id = "dv_dpi_import_prefix"
script = "dv_dpi_import_prefix.raw.py"
stage = "raw_text"
"#,
    )
    .expect("load rule");
    normalize_rule_scripts(&mut cfg, tmp_dir.path()).expect("normalize");
    infer_rule_stages(&mut cfg.rule).expect("stage");
    let paths = plugin_search_paths(&cfg, &cfg.rule[0].script);
    assert_eq!(paths[0], root.join("dv_dpi_import_prefix.raw.py"));
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
id = "dv_dpi_import_prefix"
script = "dv_dpi_import_prefix.raw.py"
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
id = "dv_dpi_import_prefix"
script = "dv_dpi_import_prefix.raw.py"
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
            assert!(detail.contains("dv_dpi_import_prefix"));
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
id = "dv_dpi_import_prefix"
script = "dv_dpi_import_prefix.raw.py"
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
id = "dv_dpi_import_prefix"
script = "dv_dpi_import_prefix.raw.py"
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
id = "dv_dpi_import_prefix"
script = "dv_dpi_import_prefix.raw.py"
stage = "raw_text"
"#,
    )
    .expect("load");
    assert!(cfg.svparser.include_paths.is_empty());
    assert!(cfg.svparser.defines.is_empty());
    assert!(!cfg.svparser.strip_comments);
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
id = "dv_dpi_import_prefix"
script = "dv_dpi_import_prefix.raw.py"
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
id = "dv_dpi_import_prefix"
script = "dv_dpi_import_prefix.raw.py"
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
id = "dv_dpi_import_prefix"
script = "a.raw.py"
stage = "raw_text"

[[rule]]
id = "dv_dpi_import_prefix"
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
id = "dv_dpi_import_prefix"
"#,
        path_str
    ))
    .expect("load");
    let err = normalize_rule_scripts(&mut cfg, tmp.path()).unwrap_err();
    assert!(matches!(err, ConfigError::InvalidValue { .. }));
}
