use crate::core::errors::ConfigError;
use crate::diag::event::{Ev, Event};
use crate::io::config::LoggingConfig;
use env_logger::Builder;
use log::LevelFilter;
use std::sync::OnceLock;

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

    let lvl = match cfg.level.as_str() {
        "error" => LevelFilter::Error,
        "warn" => LevelFilter::Warn,
        "trace" => LevelFilter::Trace,
        "debug" => LevelFilter::Debug,
        "info" => LevelFilter::Info,
        _ => LevelFilter::Info,
    };

    let mut b = Builder::new();
    b.filter(None, lvl);
    b.format(|buf, record| {
        use std::io::Write;
        writeln!(
            buf,
            "[{}] [{}] {}",
            chrono::Local::now().format("%H:%M:%S"),
            record.level(),
            record.args()
        )
    });
    let _ = b.try_init();
    Ok(())
}

fn should_emit(ev: Event) -> bool {
    if let Some(t) = TOGGLES.get() {
        match ev {
            Event::StageStart | Event::StageDone => t.stage,
            Event::PluginInvoke
            | Event::PluginDone
            | Event::PluginTimeout
            | Event::PluginExitNonzero
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
    let mut s = String::new();
    use std::fmt::Write as _;
    let _ = write!(s, "event={} path={}", e.event.name(), e.path);
    if let Some(st) = e.stage {
        let _ = write!(s, " stage={}", st);
    }
    if let Some(ms) = e.duration_ms {
        let _ = write!(s, " duration_ms={}", ms);
    }
    if let Some(code) = e.exit_code {
        let _ = write!(s, " exit_code={}", code);
    }
    if let Some(sn) = e.stderr_snippet {
        let _ = write!(s, " stderr_snippet={}", sanitize(sn));
    }
    if let Some(msg) = e.message {
        let _ = write!(s, " message={}", sanitize(msg));
    }
    log::info!("{}", s);
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
