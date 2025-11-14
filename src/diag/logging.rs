use crate::core::errors::ConfigError;
use crate::diag::event::{Ev, Event};
use crate::io::config::{LogFormat, LoggingConfig};
use std::sync::OnceLock;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::EnvFilter;

#[derive(Clone, Copy)]
struct Toggles {
    stage: bool,
    plugin: bool,
    parse: bool,
}

static TOGGLES: OnceLock<Toggles> = OnceLock::new();

pub fn init(cfg: &LoggingConfig) -> Result<(), ConfigError> {
    let _ = TOGGLES.set(Toggles {
        stage: cfg.show_stage_events,
        plugin: cfg.show_plugin_events,
        parse: cfg.show_parse_events,
    });
    for key in cfg.extra.keys() {
        tracing::warn!(target: "sv-mint::logging", "unused logging option {}", key);
    }

    let res = match cfg.format {
        LogFormat::Text => tracing_subscriber::fmt()
            .with_env_filter(build_filter(&cfg.level))
            .with_target(false)
            .finish()
            .try_init(),
        LogFormat::Json => tracing_subscriber::fmt()
            .with_env_filter(build_filter(&cfg.level))
            .with_target(false)
            .json()
            .with_current_span(false)
            .with_span_list(false)
            .finish()
            .try_init(),
    };
    res.map_err(|e| ConfigError::InvalidValue {
        detail: format!("logging init failed: {e}"),
    })
}

fn should_emit(ev: Event) -> bool {
    if let Some(t) = TOGGLES.get() {
        match ev {
            Event::StageStart | Event::StageDone => t.stage,
            Event::PluginInvoke
            | Event::PluginDone
            | Event::PluginTimeout
            | Event::PluginExitNonzero
            | Event::PluginError
            | Event::PluginStderr => t.plugin,
            Event::ParsePreprocessStart
            | Event::ParsePreprocessDone
            | Event::ParseParseStart
            | Event::ParseParseDone
            | Event::ParseAstCollectDone => t.parse,
        }
    } else {
        true
    }
}

pub fn log_event(e: Ev) {
    if !should_emit(e.event) {
        return;
    }
    let stderr = e.stderr_snippet.map(sanitize);
    let message = e.message.map(sanitize);
    tracing::info!(
        target: "sv-mint::event",
        event = e.event.name(),
        path = e.path,
        stage = e.stage.unwrap_or(""),
        duration_ms = e.duration_ms,
        exit_code = e.exit_code,
        stderr = stderr.as_deref().unwrap_or(""),
        message = message.as_deref().unwrap_or(""),
    );
}

fn sanitize(x: &str) -> String {
    let mut out = String::with_capacity(x.len());
    for ch in x.chars() {
        if ch == '\n' || ch == '\r' {
            out.push(' ');
        } else {
            out.push(ch);
        }
    }
    out
}

fn build_filter(level: &str) -> EnvFilter {
    EnvFilter::builder()
        .parse(level)
        .unwrap_or_else(|_| EnvFilter::new(default_level(level)))
}

fn default_level(level: &str) -> &str {
    match level {
        "error" | "warn" | "info" | "debug" | "trace" => level,
        _ => "info",
    }
}
