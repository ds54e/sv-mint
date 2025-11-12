use crate::core::linemap::LineMap;
use crate::sv::cst_ir::CstIr;
use serde::Serialize;
use serde_json::Value;

#[derive(Clone, Debug, Serialize)]
pub struct DefineInfo {
    pub name: String,
    pub value: Option<String>,
}

#[derive(Clone, Debug, Serialize)]
pub struct AstSummary {
    pub decls: Vec<Value>,
    pub refs: Vec<Value>,
    pub symbols: Vec<Value>,
    pub assigns: Vec<Value>,
    pub pp_text: Option<String>,
    #[serde(default)]
    pub schema_version: u32,
    #[serde(default)]
    pub scopes: Vec<Value>,
}

impl Default for AstSummary {
    fn default() -> Self {
        Self {
            decls: Vec::new(),
            refs: Vec::new(),
            symbols: Vec::new(),
            assigns: Vec::new(),
            pp_text: None,
            schema_version: 1,
            scopes: Vec::new(),
        }
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct ParseArtifacts {
    pub raw_text: String,
    pub pp_text: String,
    pub defines: Vec<DefineInfo>,
    pub has_cst: bool,
    pub ast: AstSummary,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cst_ir: Option<CstIr>,
    #[serde(skip_serializing)]
    pub line_map: LineMap,
}
