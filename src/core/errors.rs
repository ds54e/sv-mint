use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("config not found: {path}")]
    NotFound { path: String },
    #[error("invalid toml: {detail}")]
    InvalidToml { detail: String },
    #[error("invalid utf-8 in config: {path}")]
    InvalidUtf8 {
        path: String,
        #[source]
        source: Option<std::io::Error>,
    },
    #[error("invalid value: {detail}")]
    InvalidValue { detail: String },
}

#[derive(Debug, Error)]
pub enum OutputError {
    #[error("read failed: {path}")]
    ReadFailed {
        path: String,
        #[source]
        source: Option<std::io::Error>,
    },
    #[error("invalid utf-8 at: {path}")]
    InvalidUtf8 {
        path: String,
        #[source]
        source: Option<std::io::Error>,
    },
}

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("preprocess failed: {detail}")]
    PreprocessFailed { detail: String },
    #[error("parse failed: {detail}")]
    ParseFailed { detail: String },
}

#[derive(Debug, Error)]
pub enum PluginError {
    #[error("plugin spawn failed: {detail}")]
    SpawnFailed { detail: String },
    #[error("plugin io failed: {detail}")]
    IoFailed { detail: String },
    #[error("plugin timeout after {timeout_ms} ms")]
    Timeout { timeout_ms: u64 },
    #[error("plugin bad utf-8: {detail}")]
    BadUtf8 { detail: String },
    #[error("plugin bad json: {detail}")]
    BadJson { detail: String },
    #[error("plugin protocol error: {detail}")]
    ProtocolError { detail: String },
    #[error("plugin exit nonzero: code={code}")]
    ExitCode { code: i32 },
    #[error("plugin stdout too large")]
    StdoutTooLarge,
    #[error("plugin stderr too large")]
    StderrTooLarge,
}
