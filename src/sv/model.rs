use crate::core::linemap::LineMap;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct DefineInfo {
    pub name: String,
    pub value: Option<String>,
}

#[derive(Clone, Debug, Default)]
pub struct AstSummary {
    pub decls: Vec<serde_json::Value>,
    pub refs: Vec<serde_json::Value>,
    pub symbols: Vec<serde_json::Value>,
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
