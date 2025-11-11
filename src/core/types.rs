use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Location {
    pub line: u32,
    pub col: u32,
    pub end_line: u32,
    pub end_col: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Severity {
    Error,
    Warning,
    Info,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Violation {
    pub rule_id: String,
    pub severity: Severity,
    pub message: String,
    pub location: Location,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Stage {
    RawText,
    PpText,
    Cst,
    Ast,
}

impl Stage {
    pub fn as_str(&self) -> &'static str {
        match self {
            Stage::RawText => "raw_text",
            Stage::PpText => "pp_text",
            Stage::Cst => "cst",
            Stage::Ast => "ast",
        }
    }
}
