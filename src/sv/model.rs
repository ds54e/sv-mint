use crate::core::linemap::LineMap;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DefineInfo {
    pub name: String,
    pub value: Option<String>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct AstSummary {
    pub decls: Vec<serde_json::Value>,
    pub refs: Vec<serde_json::Value>,
    pub symbols: Vec<serde_json::Value>,
    pub assigns: Vec<serde_json::Value>,
    pub pp_text: Option<String>,
}

#[derive(Clone, Debug)]
pub struct ParseArtifacts {
    pub raw_text: String,
    pub pp_text: String,
    pub defines: Vec<DefineInfo>,
    pub has_cst: bool,
    pub ast: AstSummary,
    pub line_map: LineMap,
}
