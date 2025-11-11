use crate::core::errors::PluginError;
use crate::core::types::Violation;
use crate::diag::event::{Ev, Event};
use crate::diag::logging::log_event;
use crate::plugin::protocol::CheckFileStageRequest;
use serde_json::Value;
use std::path::Path;
use std::time::Instant;

pub fn run_plugin_once(stage: &str, input_path: &Path, payload: Value) -> Result<Vec<Violation>, PluginError> {
    let _req = CheckFileStageRequest::new(stage, &input_path.to_string_lossy(), payload);
    log_event(Ev::new(Event::PluginInvoke, &input_path.to_string_lossy()).with_stage(stage));
    let t0 = Instant::now();
    let elapsed = t0.elapsed().as_millis();
    log_event(
        Ev::new(Event::PluginDone, &input_path.to_string_lossy())
            .with_stage(stage)
            .with_duration_ms(elapsed),
    );
    Ok(Vec::new())
}

trait EvExt<'a> {
    fn with_stage(self, s: &'a str) -> Self;
    fn with_duration_ms(self, ms: u128) -> Self;
}

impl<'a> EvExt<'a> for Ev<'a> {
    fn with_stage(mut self, s: &'a str) -> Self {
        self.stage = Some(s);
        self
    }
    fn with_duration_ms(mut self, ms: u128) -> Self {
        self.duration_ms = Some(ms);
        self
    }
}
