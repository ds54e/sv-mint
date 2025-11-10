use thiserror::Error;

#[allow(dead_code)]
#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("config not found: {path}")]
    NotFound { path: String },
    #[error("invalid toml: {detail}")]
    InvalidToml { detail: String },
    #[error("invalid utf-8 in config: {path}")]
    InvalidUtf8 { path: String },
    #[error("invalid value: {detail}")]
    InvalidValue { detail: String },
}

#[allow(dead_code)]
#[derive(Debug, Error)]
pub enum ParseError {
    #[error("preprocess failed: {detail}")]
    PreprocessFailed { detail: String },
    #[error("parse failed: {detail}")]
    ParseFailed { detail: String },
}

#[allow(dead_code)]
#[derive(Debug, Error)]
pub enum PluginError {
    #[error("plugin spawn failed: {detail}")]
    SpawnFailed {
        detail: String,
        #[source]
        source: Option<std::io::Error>,
    },
    #[error("plugin io failed: {detail}")]
    IoFailed {
        detail: String,
        #[source]
        source: Option<std::io::Error>,
    },
    #[error("plugin timeout after {timeout_ms} ms")]
    Timeout { timeout_ms: u64 },
    #[error("plugin bad utf-8")]
    BadUtf8,
    #[error("plugin returned bad json: {detail}")]
    BadJson { detail: String },
    #[error("plugin protocol error: {detail}")]
    ProtocolError { detail: String },
    #[error("plugin exited with non-zero code: {code}")]
    ExitCode { code: i32 },
    #[error("plugin stdout too large")]
    StdoutTooLarge,
    #[error("plugin stderr too large")]
    StderrTooLarge,
}

#[allow(dead_code)]
#[derive(Debug, Error)]
pub enum OutputError {
    #[error("read failed: {path}")]
    ReadFailed {
        path: String,
        #[source]
        source: Option<std::io::Error>,
    },
    #[error("invalid utf-8: {path}")]
    InvalidUtf8 { path: String },
}

#[allow(dead_code)]
pub type AppResult<T> = Result<T, anyhow::Error>;
