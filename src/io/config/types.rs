use crate::svparser::SvParserCfg;
use crate::types::Stage;
use serde::Deserialize;
use std::collections::HashMap;
use std::path::PathBuf;
use toml::Value as TomlValue;

#[derive(Deserialize, Clone)]
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
    pub enabled: Vec<Stage>,
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

fn default_warn_margin_bytes() -> usize {
    1_048_576
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
