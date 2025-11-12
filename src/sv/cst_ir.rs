use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
pub struct CstIncludeFlags {
    pub text: bool,
    pub tokens: bool,
}

#[derive(Clone, Debug, Serialize)]
pub struct TokenRec {
    pub id: u32,
    pub kind: u16,
    pub start: u32,
    pub end: u32,
}

#[derive(Clone, Debug, Serialize)]
pub struct NodeRec {
    pub id: u32,
    pub kind: u16,
    pub start: u32,
    pub end: u32,
    pub parent: Option<u32>,
    pub first_token: u32,
    pub last_token: u32,
}

#[derive(Clone, Debug, Serialize)]
pub struct CstIr {
    pub schema: u32,
    pub format: &'static str,
    pub sv_parser: String,
    pub file: String,
    pub hash: String,
    pub line_starts: Vec<u32>,
    pub include: CstIncludeFlags,
    pub pp_text: Option<String>,
    pub kind_table: Vec<String>,
    pub tok_kind_table: Vec<String>,
    pub tokens: Vec<TokenRec>,
    pub nodes: Vec<NodeRec>,
}

pub fn build_cst_ir_stub(file: &str, sv_parser_ver: &str, line_starts: &[usize], pp_text: &str) -> CstIr {
    CstIr {
        schema: 1,
        format: "json",
        sv_parser: sv_parser_ver.to_string(),
        file: file.to_string(),
        hash: String::new(),
        line_starts: line_starts.iter().map(|&x| x as u32).collect(),
        include: CstIncludeFlags {
            text: true,
            tokens: true,
        },
        pp_text: Some(pp_text.to_string()),
        kind_table: Vec::new(),
        tok_kind_table: Vec::new(),
        tokens: Vec::new(),
        nodes: Vec::new(),
    }
}
