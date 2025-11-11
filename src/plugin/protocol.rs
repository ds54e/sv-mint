use crate::core::types::Violation;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize)]
pub struct CheckFileStageRequest<'a> {
    #[serde(rename = "type")]
    pub ty: &'static str,
    pub stage: &'a str,
    pub path: &'a str,
    pub payload: Value,
}

impl<'a> CheckFileStageRequest<'a> {
    pub fn new(stage: &'a str, path: &'a str, payload: Value) -> Self {
        Self {
            ty: "CheckFileStage",
            stage,
            path,
            payload,
        }
    }
}

#[derive(Deserialize)]
pub struct ViolationsStageResponse {
    #[serde(rename = "type")]
    pub ty: String,
    pub stage: String,
    pub violations: Vec<Violation>,
}
