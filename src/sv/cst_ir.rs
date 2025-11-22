use serde::Serialize;
use std::collections::HashMap;
use sv_parser::{Locate, NodeEvent, RefNode, SyntaxTree};

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
    pub text: String,
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
    pub source_text: String,
    pub line_starts: Vec<u32>,
    pub include: CstIncludeFlags,
    pub pp_text: Option<String>,
    pub kind_table: Vec<String>,
    pub tok_kind_map: std::collections::HashMap<String, u16>,
    pub tok_kind_table: Vec<String>,
    pub tokens: Vec<TokenRec>,
    pub nodes: Vec<NodeRec>,
}

pub fn build_cst_ir(tree: &SyntaxTree, file: &str, sv_parser_ver: &str, line_starts: &[usize], pp_text: &str) -> CstIr {
    let mut builder = CstBuilder::new(file, sv_parser_ver, line_starts, pp_text);
    builder.walk(tree);
    builder.finish()
}

struct NodeState {
    id: u32,
    kind: u16,
    parent: Option<u32>,
    start: Option<u32>,
    end: Option<u32>,
    first_token: Option<u32>,
    last_token: Option<u32>,
}

struct CstBuilder {
    file: String,
    sv_parser: String,
    line_starts: Vec<u32>,
    source_text: String,
    pp_text: String,
    kind_table: Vec<String>,
    kind_map: HashMap<String, u16>,
    tok_kind_table: Vec<String>,
    tok_kind_map: HashMap<String, u16>,
    nodes: Vec<NodeRec>,
    tokens: Vec<TokenRec>,
    stack: Vec<NodeState>,
    next_node_id: u32,
}

impl CstBuilder {
    fn new(file: &str, sv_parser: &str, line_starts: &[usize], pp_text: &str) -> Self {
        Self {
            file: file.to_string(),
            sv_parser: sv_parser.to_string(),
            line_starts: line_starts.iter().map(|&x| x as u32).collect(),
            source_text: pp_text.to_string(),
            pp_text: pp_text.to_string(),
            kind_table: Vec::new(),
            kind_map: HashMap::new(),
            tok_kind_table: Vec::new(),
            tok_kind_map: HashMap::new(),
            nodes: Vec::new(),
            tokens: Vec::new(),
            stack: Vec::new(),
            next_node_id: 0,
        }
    }

    fn walk(&mut self, tree: &SyntaxTree) {
        for event in tree.into_iter().event() {
            match event {
                NodeEvent::Enter(RefNode::Locate(loc)) => {
                    self.record_token(tree, loc);
                }
                NodeEvent::Leave(RefNode::Locate(_)) => {}
                NodeEvent::Enter(node) => {
                    self.enter_node(&node);
                }
                NodeEvent::Leave(_) => {
                    self.leave_node();
                }
            }
        }
    }

    fn finish(self) -> CstIr {
        CstIr {
            schema: 2,
            format: "json",
            sv_parser: self.sv_parser,
            file: self.file,
            hash: String::new(),
            source_text: self.source_text,
            line_starts: self.line_starts,
            include: CstIncludeFlags {
                text: true,
                tokens: true,
            },
            pp_text: Some(self.pp_text),
            kind_table: self.kind_table,
            tok_kind_map: self.tok_kind_map,
            tok_kind_table: self.tok_kind_table,
            tokens: self.tokens,
            nodes: self.nodes,
        }
    }

    fn enter_node(&mut self, node: &RefNode<'_>) {
        if matches!(node, RefNode::Locate(_)) {
            return;
        }
        let name = ref_node_name(node);
        let kind = self.kind_id(&name);
        let parent = self.stack.last().map(|s| s.id);
        let id = self.next_node_id;
        self.next_node_id += 1;
        self.stack.push(NodeState {
            id,
            kind,
            parent,
            start: None,
            end: None,
            first_token: None,
            last_token: None,
        });
    }

    fn leave_node(&mut self) {
        if let Some(state) = self.stack.pop() {
            let Some(first) = state.first_token else {
                return;
            };
            let Some(last) = state.last_token else {
                return;
            };
            let start = state.start.unwrap_or(0);
            let end = state.end.unwrap_or(start);
            self.nodes.push(NodeRec {
                id: state.id,
                kind: state.kind,
                start,
                end,
                parent: state.parent,
                first_token: first,
                last_token: last,
            });
        }
    }

    fn record_token(&mut self, tree: &SyntaxTree, loc: &Locate) {
        if let Some(text) = tree.get_str(loc) {
            let kind_name = classify_token(text);
            let kind = self.tok_kind_id(&kind_name);
            let start = loc.offset as u32;
            let end = (loc.offset + loc.len) as u32;
            let id = self.tokens.len() as u32;
            self.tokens.push(TokenRec {
                id,
                kind,
                start,
                end,
                text: text.to_string(),
            });
            for state in self.stack.iter_mut().rev() {
                if state.start.is_none() {
                    state.start = Some(start);
                }
                state.end = Some(end);
                if state.first_token.is_none() {
                    state.first_token = Some(id);
                }
                state.last_token = Some(id);
            }
        }
    }

    fn kind_id(&mut self, name: &str) -> u16 {
        if let Some(id) = self.kind_map.get(name) {
            *id
        } else {
            let id = self.kind_table.len() as u16;
            self.kind_table.push(name.to_string());
            self.kind_map.insert(name.to_string(), id);
            id
        }
    }

    fn tok_kind_id(&mut self, name: &str) -> u16 {
        if let Some(id) = self.tok_kind_map.get(name) {
            *id
        } else {
            let id = self.tok_kind_table.len() as u16;
            self.tok_kind_table.push(name.to_string());
            self.tok_kind_map.insert(name.to_string(), id);
            id
        }
    }
}

fn ref_node_name(node: &RefNode<'_>) -> String {
    let raw = format!("{node:?}");
    raw.split('(').next().unwrap_or("RefNode").to_string()
}

fn classify_token(text: &str) -> String {
    let trimmed = text.trim();
    if trimmed.is_empty() {
        return "ws".to_string();
    }
    match trimmed {
        ".*" => return "conn_wildcard".to_string(),
        "." => return "dot".to_string(),
        "(" => return "paren_open".to_string(),
        ")" => return "paren_close".to_string(),
        "," => return "comma".to_string(),
        ";" => return "semicolon".to_string(),
        ":" => return "colon".to_string(),
        "=" => return "op_eq".to_string(),
        "<=" => return "op_le".to_string(),
        "@" => return "at".to_string(),
        _ => {}
    }
    let lower = trimmed.to_ascii_lowercase();
    match lower.as_str() {
        "always_ff" => "kw_always_ff".to_string(),
        "always_comb" => "kw_always_comb".to_string(),
        "always_latch" => "kw_always_latch".to_string(),
        "begin" => "kw_begin".to_string(),
        "end" => "kw_end".to_string(),
        "case" | "casez" | "casex" => "kw_case".to_string(),
        "endcase" => "kw_endcase".to_string(),
        "default" => "kw_default".to_string(),
        "unique" => "kw_unique".to_string(),
        "priority" => "kw_priority".to_string(),
        _ => {
            if trimmed.starts_with("//") {
                "line_comment".to_string()
            } else if trimmed.starts_with("/*") {
                "block_comment".to_string()
            } else if trimmed.starts_with('`') {
                "macro_id".to_string()
            } else if trimmed.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') {
                "ident".to_string()
            } else {
                "symbol".to_string()
            }
        }
    }
}
