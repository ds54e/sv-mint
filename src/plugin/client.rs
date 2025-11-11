use crate::core::errors::PluginError;
use crate::core::types::Violation;
use crate::plugin::protocol::CheckFileStageRequest;
use log::info;
use serde_json::Value;
use std::path::Path;

pub fn run_plugin_once(stage: &str, input_path: &Path, payload: Value) -> Result<Vec<Violation>, PluginError> {
    let _req = CheckFileStageRequest::new(stage, &input_path.to_string_lossy(), payload);
    info!("event=plugin_invoke stage={} path={}", stage, input_path.display());
    Ok(Vec::new())
}
