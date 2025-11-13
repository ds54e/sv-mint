use crate::sv::cst_ir::CstIr;
use crate::types::Location;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SvParserCfg {
    #[serde(default)]
    pub include_paths: Vec<String>,
    #[serde(default)]
    pub defines: Vec<String>,
    #[serde(default)]
    pub strip_comments: bool,
    #[serde(default)]
    pub ignore_include: bool,
    #[serde(default)]
    pub allow_incomplete: bool,
}

impl Default for SvParserCfg {
    fn default() -> Self {
        Self {
            include_paths: Vec::new(),
            defines: Vec::new(),
            strip_comments: true,
            ignore_include: true,
            allow_incomplete: true,
        }
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct DefineInfo {
    pub name: String,
    pub value: Option<String>,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum DeclKind {
    Module,
    Param,
    Net,
    Var,
}

#[derive(Clone, Debug, Serialize)]
pub struct Declaration {
    pub kind: DeclKind,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub module: Option<String>,
    pub loc: Location,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ReferenceKind {
    Read,
    Write,
}

#[derive(Clone, Debug, Serialize)]
pub struct Reference {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub module: Option<String>,
    pub kind: ReferenceKind,
    pub loc: Location,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AssignOp {
    BlockingOrCont,
    Nonblocking,
}

#[derive(Clone, Debug, Serialize)]
pub struct Assignment {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub module: Option<String>,
    pub op: AssignOp,
    pub lhs: String,
    pub rhs: String,
    pub loc: Location,
}

#[derive(Clone, Debug, Serialize)]
pub struct PortInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub module: Option<String>,
    pub name: String,
    pub direction: String,
    pub loc: Location,
}

#[derive(Clone, Debug, Serialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum SymbolClass {
    Param,
    Net,
    Var,
}

#[derive(Clone, Debug, Serialize)]
pub struct SymbolUsage {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub module: Option<String>,
    pub name: String,
    pub class: SymbolClass,
    pub ref_count: usize,
    pub read_count: usize,
    pub write_count: usize,
    pub used: bool,
    pub loc: Location,
}

#[derive(Clone, Debug, Serialize, Default)]
pub struct ScopeInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
}

#[derive(Clone, Debug, Serialize)]
pub struct AstSummary {
    pub decls: Vec<Declaration>,
    pub refs: Vec<Reference>,
    pub symbols: Vec<SymbolUsage>,
    pub assigns: Vec<Assignment>,
    #[serde(default)]
    pub ports: Vec<PortInfo>,
    pub pp_text: Option<String>,
    #[serde(default)]
    pub schema_version: u32,
    #[serde(default)]
    pub scopes: Vec<ScopeInfo>,
}

impl Default for AstSummary {
    fn default() -> Self {
        Self {
            decls: Vec::new(),
            refs: Vec::new(),
            symbols: Vec::new(),
            assigns: Vec::new(),
            ports: Vec::new(),
            pp_text: None,
            schema_version: 1,
            scopes: Vec::new(),
        }
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct ParseArtifacts {
    pub raw_text: String,
    pub normalized_text: String,
    pub pp_text: String,
    pub defines: Vec<DefineInfo>,
    pub has_cst: bool,
    pub ast: AstSummary,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cst_ir: Option<CstIr>,
}
