use serde::Serialize;
use serde_json::{Map as JsonMap, Value as JsonValue};
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
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub children: Vec<u32>,
    #[serde(skip_serializing_if = "JsonMap::is_empty")]
    pub fields: JsonMap<String, JsonValue>,
}

#[derive(Clone, Debug, Serialize)]
pub struct DirectiveRec {
    pub kind: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<u32>,
    #[serde(skip_serializing_if = "is_zero")]
    pub depth: u32,
    pub token: u32,
    pub start: u32,
    pub end: u32,
}

fn is_zero(v: &u32) -> bool {
    *v == 0
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
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub directives: Vec<DirectiveRec>,
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
    children: Vec<u32>,
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
    nodes: Vec<Option<NodeRec>>,
    tokens: Vec<TokenRec>,
    directives: Vec<DirectiveRec>,
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
            directives: Vec::new(),
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
                NodeEvent::Leave(node) => {
                    self.leave_node(&node);
                }
            }
        }
    }

    fn finish(self) -> CstIr {
        let mut this = self;
        this.fill_directives_from_text();
        this.apply_directive_nesting();
        CstIr {
            schema: 2,
            format: "json",
            sv_parser: this.sv_parser,
            file: this.file,
            hash: String::new(),
            source_text: this.source_text,
            line_starts: this.line_starts,
            include: CstIncludeFlags {
                text: true,
                tokens: true,
            },
            pp_text: Some(this.pp_text),
            kind_table: this.kind_table,
            tok_kind_map: this.tok_kind_map,
            tok_kind_table: this.tok_kind_table,
            tokens: this.tokens,
            nodes: this.nodes.into_iter().flatten().collect(),
            directives: this.directives,
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
            children: Vec::new(),
        });
    }

    fn leave_node(&mut self, node: &RefNode<'_>) {
        if let Some(mut state) = self.stack.pop() {
            if let Some(parent) = self.stack.last_mut() {
                parent.children.push(state.id);
            }
            let mut first = state.first_token;
            let mut last = state.last_token;
            if (first.is_none() || last.is_none()) && self.should_force_node(node) {
                if let Some(parent) = self.stack.iter().rev().find(|s| s.first_token.is_some()) {
                    if first.is_none() {
                        first = parent.first_token;
                    }
                    if last.is_none() {
                        last = parent.last_token;
                    }
                    if state.start.is_none() {
                        state.start = parent.start;
                    }
                    if state.end.is_none() {
                        state.end = parent.end;
                    }
                }
            }
            let Some(first) = first else {
                return;
            };
            let Some(last) = last else {
                return;
            };
            let start = state.start.unwrap_or(0);
            let end = state.end.unwrap_or(start);
            let fields = self.build_fields(node, &state);
            let record = NodeRec {
                id: state.id,
                kind: state.kind,
                start,
                end,
                parent: state.parent,
                first_token: first,
                last_token: last,
                children: state.children,
                fields,
            };
            if self.nodes.len() <= state.id as usize {
                self.nodes.resize(state.id as usize + 1, None);
            }
            self.nodes[state.id as usize] = Some(record);
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
            self.maybe_record_directive(id, start, end, text);
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

    fn maybe_record_directive(&mut self, token: u32, start: u32, end: u32, text: &str) {
        let trimmed = text.trim();
        if !trimmed.starts_with('`') {
            return;
        }
        let mut parts = trimmed.split_whitespace();
        let Some(first) = parts.next() else {
            return;
        };
        if first.len() <= 1 {
            return;
        }
        let kind = first[1..].to_ascii_lowercase();
        if kind.is_empty() {
            return;
        }
        let value = parts.next().map(|s| s.trim_matches(|c| c == ';').to_ascii_lowercase());
        self.directives.push(DirectiveRec {
            kind,
            value,
            parent: None,
            depth: 0,
            token,
            start,
            end,
        });
    }

    fn fill_directives_from_text(&mut self) {
        let lower = self.source_text.to_ascii_lowercase();
        let mut pos = 0usize;
        let needle = "`default_nettype";
        while let Some(idx) = lower[pos..].find(needle) {
            let abs = pos + idx;
            let rest_start = abs + needle.len();
            let rest = self.source_text[rest_start..].trim_start();
            let value = rest.split_whitespace().next();
            let end_rel = rest.find('\n').unwrap_or(rest.len());
            let end = (rest_start + end_rel) as u32;
            let val = value.map(|v| v.trim_matches(|c| c == ';').to_ascii_lowercase());
            if !self
                .directives
                .iter()
                .any(|d| d.start == abs as u32 && d.kind == "default_nettype")
            {
                self.directives.push(DirectiveRec {
                    kind: "default_nettype".to_string(),
                    value: val,
                    parent: None,
                    depth: 0,
                    token: 0,
                    start: abs as u32,
                    end,
                });
            }
            pos = rest_start;
        }
    }

    fn apply_directive_nesting(&mut self) {
        self.directives.sort_by_key(|d| d.start);
        self.directives
            .dedup_by(|a, b| a.start == b.start && a.kind == b.kind && a.value == b.value);
        let mut stack: Vec<usize> = Vec::new();
        for (i, dir) in self.directives.iter_mut().enumerate() {
            dir.parent = stack.last().copied().map(|idx| idx as u32);
            dir.depth = stack.len() as u32;
            match dir.kind.as_str() {
                "ifdef" | "ifndef" => {
                    stack.push(i);
                }
                "elsif" | "else" => {
                    // stay at same depth
                }
                "endif" => {
                    dir.parent = stack.last().copied().map(|idx| idx as u32);
                    dir.depth = stack.len() as u32;
                    stack.pop();
                }
                _ => {}
            }
        }
    }

    fn should_force_node(&self, node: &RefNode<'_>) -> bool {
        matches!(
            node,
            RefNode::DataTypeOrImplicit(_) | RefNode::FunctionDataTypeOrImplicit(_) | RefNode::ImplicitDataType(_)
        )
    }

    fn build_fields(&self, node: &RefNode<'_>, state: &NodeState) -> JsonMap<String, JsonValue> {
        match node {
            RefNode::FunctionDataTypeOrImplicit(_) | RefNode::DataTypeOrImplicit(_) => self.build_type_field(state),
            RefNode::FunctionDeclaration(_) => self.build_function_fields(state),
            RefNode::ParameterDeclaration(_) | RefNode::LocalParameterDeclaration(_) => self.build_param_fields(state),
            RefNode::TfPortItem(_) => self.build_tf_port_item(state),
            RefNode::AlwaysConstruct(_) => self.build_always_fields(state),
            RefNode::CaseStatement(_) => self.build_case_fields(state),
            RefNode::HierarchicalInstance(_) => self.build_instance_fields(state),
            _ => JsonMap::new(),
        }
    }

    fn build_type_field(&self, state: &NodeState) -> JsonMap<String, JsonValue> {
        let mut fields = JsonMap::new();
        if let Some(ty) = self.type_from_children(&state.children) {
            fields.insert("type".to_string(), JsonValue::from(ty));
        }
        fields
    }

    fn build_always_fields(&self, state: &NodeState) -> JsonMap<String, JsonValue> {
        let mut fields = JsonMap::new();
        let Some(first) = state.first_token else {
            return fields;
        };
        let Some(last) = state.last_token else {
            return fields;
        };
        let kind = self.always_kind(first, last);
        fields.insert("always_kind".to_string(), JsonValue::String(kind));
        let events = self.always_events(&state.children, first, last);
        if !events.is_empty() {
            fields.insert("events".to_string(), JsonValue::Array(events));
        }
        fields
    }

    fn build_case_fields(&self, state: &NodeState) -> JsonMap<String, JsonValue> {
        let mut fields = JsonMap::new();
        let Some(first) = state.first_token else {
            return fields;
        };
        let Some(last) = state.last_token else {
            return fields;
        };
        let mut has_default = false;
        let mut is_unique = false;
        let mut is_priority = false;
        for tok in first..=last {
            let word = self.token_text(tok);
            match word {
                w if w.eq_ignore_ascii_case("default") => {
                    has_default = true;
                }
                w if w.eq_ignore_ascii_case("unique") || w.eq_ignore_ascii_case("unique0") => {
                    is_unique = true;
                }
                w if w.eq_ignore_ascii_case("priority") => {
                    is_priority = true;
                }
                _ => {}
            }
        }
        fields.insert("has_default".to_string(), JsonValue::Bool(has_default));
        fields.insert("is_unique".to_string(), JsonValue::Bool(is_unique));
        fields.insert("is_priority".to_string(), JsonValue::Bool(is_priority));
        fields
    }

    fn build_instance_fields(&self, state: &NodeState) -> JsonMap<String, JsonValue> {
        let mut fields = JsonMap::new();
        let mut name_token = None;
        let mut connections = Vec::new();
        self.collect_descendants_of_kind(&state.children, "NameOfInstance", &mut connections);
        if let Some(id) = connections.first() {
            if let Some(node) = self.node(*id) {
                name_token = Some(node.first_token);
            }
        }
        connections.clear();
        self.collect_descendants_of_kind(&state.children, "NamedPortConnectionIdentifier", &mut connections);
        self.collect_descendants_of_kind(&state.children, "OrderedPortConnection", &mut connections);
        let mut conn_fields = Vec::new();
        for id in connections {
            if let Some(node) = self.node(id) {
                conn_fields.push(JsonValue::Object(self.connection_entry(id, node)));
            }
        }
        let mut param_fields = Vec::new();
        let mut params = Vec::new();
        self.collect_descendants_of_kind(&state.children, "NamedParameterAssignment", &mut params);
        self.collect_descendants_of_kind(&state.children, "OrderedParameterAssignment", &mut params);
        for id in params {
            if let Some(node) = self.node(id) {
                param_fields.push(JsonValue::Object(self.connection_entry(id, node)));
            }
        }
        if let Some(tok) = name_token {
            fields.insert("name_token".to_string(), JsonValue::from(tok));
        }
        if !conn_fields.is_empty() {
            fields.insert("connections".to_string(), JsonValue::Array(conn_fields));
        }
        if !param_fields.is_empty() {
            fields.insert("param_connections".to_string(), JsonValue::Array(param_fields));
        }
        fields
    }

    fn build_param_fields(&self, state: &NodeState) -> JsonMap<String, JsonValue> {
        let mut fields = JsonMap::new();
        if let Some(id) = self.find_descendant_of_kind(&state.children, "ParameterDeclarationType") {
            fields.insert("type".to_string(), JsonValue::from(id));
        } else if let Some(id) = self.find_descendant_of_kind(&state.children, "LocalParameterDeclarationType") {
            fields.insert("type".to_string(), JsonValue::from(id));
        } else if let Some(dtype) = self.find_descendant_of_kind(&state.children, "DataTypeOrImplicit") {
            if let Some(node) = self.node(dtype) {
                if let Some(ty) = self.type_from_children(&node.children) {
                    fields.insert("type".to_string(), JsonValue::from(ty));
                }
            }
        } else if let Some(ty) = self.find_type_in_descendants(&state.children) {
            fields.insert("type".to_string(), JsonValue::from(ty));
        }
        fields
    }

    fn build_function_fields(&self, state: &NodeState) -> JsonMap<String, JsonValue> {
        let mut fields = JsonMap::new();
        if let Some(holder) = self.find_descendant_of_kind(&state.children, "FunctionDataTypeOrImplicit") {
            if let Some(node) = self.node(holder) {
                if let Some(ty) = self.type_from_children(&node.children) {
                    fields.insert("return_type".to_string(), JsonValue::from(ty));
                }
            }
        } else if let Some(ty) = self.find_type_in_descendants(&state.children) {
            fields.insert("return_type".to_string(), JsonValue::from(ty));
        }
        let ports = self.collect_ports(&state.children);
        if !ports.is_empty() {
            fields.insert("ports".to_string(), JsonValue::Array(ports));
        }
        fields
    }

    fn build_tf_port_item(&self, state: &NodeState) -> JsonMap<String, JsonValue> {
        let mut port = JsonMap::new();
        if let Some(dir) = self.port_direction(state) {
            port.insert("dir".to_string(), JsonValue::String(dir));
        } else {
            port.insert("dir".to_string(), JsonValue::Null);
        }
        let ty_id = self.port_type_id(state);
        if let Some(ty) = ty_id {
            port.insert("type".to_string(), JsonValue::from(ty));
        } else {
            port.insert("type".to_string(), JsonValue::Null);
        }
        let mut name_token = self.find_child_token(state, "PortIdentifier");
        if name_token.is_none() {
            if let Some(ty) = ty_id {
                if let Some(node) = self.node(ty) {
                    name_token = Some(node.first_token);
                }
            }
        }
        if let Some(tok) = name_token {
            port.insert("name_token".to_string(), JsonValue::from(tok));
        } else {
            port.insert("name_token".to_string(), JsonValue::Null);
        }
        if let Some(expr) = self.find_child_of_kind(state, "Expression") {
            port.insert("expr".to_string(), JsonValue::from(expr));
        }
        let mut fields = JsonMap::new();
        fields.insert("port".to_string(), JsonValue::Object(port));
        fields
    }

    fn port_direction(&self, state: &NodeState) -> Option<String> {
        for &child in &state.children {
            let node = self.node(child)?;
            let name = self.kind_name(node.kind);
            if name == "TfPortDirection" || name == "PortDirection" {
                return self.direction_from_tokens(node.first_token, node.last_token);
            }
        }
        None
    }

    fn direction_from_tokens(&self, first: u32, last: u32) -> Option<String> {
        for tok in first..=last {
            let text = self.tokens.get(tok as usize)?.text.trim().to_ascii_lowercase();
            match text.as_str() {
                "input" | "output" | "inout" | "ref" => return Some(text),
                "const" => return Some("const_ref".to_string()),
                _ => {}
            }
        }
        None
    }

    fn find_child_token(&self, state: &NodeState, kind: &str) -> Option<u32> {
        let id = self.find_child_of_kind(state, kind)?;
        Some(self.node(id)?.first_token)
    }

    fn find_child_of_kind(&self, state: &NodeState, kind: &str) -> Option<u32> {
        for &child in &state.children {
            let node = self.node(child)?;
            if self.kind_name(node.kind) == kind {
                return Some(child);
            }
        }
        None
    }

    fn port_type_id(&self, state: &NodeState) -> Option<u32> {
        if let Some(holder) = self.find_child_of_kind(state, "DataTypeOrImplicit") {
            if let Some(node) = self.node(holder) {
                if let Some(ty) = self.type_from_children(&node.children) {
                    return Some(ty);
                }
            }
        }
        if let Some(id) = self.find_child_of_kind(state, "ImplicitDataType") {
            return Some(id);
        }
        self.find_type_in_descendants(&state.children)
    }

    fn collect_ports(&self, roots: &[u32]) -> Vec<JsonValue> {
        let mut ids = Vec::new();
        self.collect_descendants_of_kind(roots, "TfPortItem", &mut ids);
        let mut ports = Vec::new();
        for id in ids {
            if let Some(node) = self.node(id) {
                if let Some(port) = node.fields.get("port") {
                    ports.push(port.clone());
                }
            }
        }
        ports
    }

    fn collect_descendants_of_kind(&self, roots: &[u32], kind: &str, out: &mut Vec<u32>) {
        for &child in roots {
            let Some(node) = self.node(child) else {
                continue;
            };
            if self.kind_name(node.kind) == kind {
                out.push(child);
            }
            self.collect_descendants_of_kind(&node.children, kind, out);
        }
    }

    fn find_type_in_descendants(&self, roots: &[u32]) -> Option<u32> {
        if let Some(ty) = self.type_from_children(roots) {
            return Some(ty);
        }
        for &child in roots {
            let Some(node) = self.node(child) else {
                continue;
            };
            if let Some(val) = node.fields.get("type").and_then(|v| v.as_u64()) {
                return Some(val as u32);
            }
            if let Some(ty) = self.find_type_in_descendants(&node.children) {
                return Some(ty);
            }
        }
        None
    }

    fn find_descendant_of_kind(&self, roots: &[u32], kind: &str) -> Option<u32> {
        for &child in roots {
            let Some(node) = self.node(child) else {
                continue;
            };
            if self.kind_name(node.kind) == kind {
                return Some(child);
            }
            if let Some(found) = self.find_descendant_of_kind(&node.children, kind) {
                return Some(found);
            }
        }
        None
    }

    fn token_text(&self, tok: u32) -> &str {
        self.tokens.get(tok as usize).map(|t| t.text.trim()).unwrap_or("")
    }

    fn find_token_by_text(&self, first: u32, last: u32, text: &str) -> Option<u32> {
        (first..=last).find(|&tok| self.token_text(tok).eq_ignore_ascii_case(text))
    }

    fn always_kind(&self, first: u32, last: u32) -> String {
        for tok in first..=last {
            let word = self.token_text(tok).to_ascii_lowercase();
            if word.is_empty() {
                continue;
            }
            return match word.as_str() {
                "always_ff" => "ff".to_string(),
                "always_comb" => "comb".to_string(),
                "always_latch" => "latch".to_string(),
                _ => "always".to_string(),
            };
        }
        "always".to_string()
    }

    fn always_events(&self, roots: &[u32], first: u32, last: u32) -> Vec<JsonValue> {
        let mut ids = Vec::new();
        self.collect_descendants_of_kind(roots, "EventExpressionExpression", &mut ids);
        let mut events = Vec::new();
        for id in ids {
            let Some(node) = self.node(id) else {
                continue;
            };
            if node.first_token < first || node.last_token > last {
                continue;
            }
            let mut separator = String::new();
            let mut parent_id = node.parent;
            while let Some(pid) = parent_id {
                let Some(parent) = self.node(pid) else {
                    break;
                };
                let name = self.kind_name(parent.kind);
                if name == "EventExpressionOr" {
                    separator = "or".to_string();
                    break;
                }
                if name == "EventExpressionComma" {
                    separator = "comma".to_string();
                    break;
                }
                parent_id = parent.parent;
            }
            let expr = self.find_descendant_of_kind(&node.children, "Expression");
            let edge = self
                .find_descendant_of_kind(&node.children, "EdgeIdentifier")
                .map(|edge_id| {
                    self.token_text(self.node(edge_id).map(|n| n.first_token).unwrap_or(node.first_token))
                        .to_ascii_lowercase()
                });
            let mut m = JsonMap::new();
            let token_id = if separator.is_empty() {
                node.first_token
            } else {
                node.parent
                    .and_then(|p| self.node(p))
                    .and_then(|p| self.find_token_by_text(p.first_token, p.last_token, &separator))
                    .unwrap_or(node.first_token)
            };
            if !separator.is_empty() {
                m.insert("separator".to_string(), JsonValue::String(separator));
            }
            m.insert("token".to_string(), JsonValue::from(token_id));
            if let Some(expr_id) = expr {
                m.insert("expr".to_string(), JsonValue::from(expr_id));
            }
            if let Some(edge) = edge {
                m.insert("edge".to_string(), JsonValue::String(edge));
            }
            events.push(JsonValue::Object(m));
        }
        events
    }

    fn connection_entry(&self, id: u32, node: &NodeRec) -> JsonMap<String, JsonValue> {
        let kind = self.kind_name(node.kind);
        let named = kind == "NamedPortConnectionIdentifier";
        let name_token = if named {
            self.find_descendant_of_kind(&[id], "PortIdentifier")
                .and_then(|n| self.node(n).map(|nn| nn.first_token))
        } else {
            None
        };
        let expr = self.find_descendant_of_kind(&[id], "Expression");
        let mut m = JsonMap::new();
        m.insert("named".to_string(), JsonValue::Bool(named));
        m.insert("token".to_string(), JsonValue::from(node.first_token));
        if let Some(tok) = name_token {
            m.insert("name_token".to_string(), JsonValue::from(tok));
        }
        if let Some(expr_id) = expr {
            m.insert("expr".to_string(), JsonValue::from(expr_id));
        }
        m
    }

    fn type_from_children(&self, children: &[u32]) -> Option<u32> {
        for &child in children {
            let Some(node) = self.node(child) else {
                continue;
            };
            let name = self.kind_name(node.kind);
            if name == "DataType" || name == "NetType" || name == "ImplicitDataType" {
                return Some(child);
            }
        }
        for &child in children {
            let Some(node) = self.node(child) else {
                continue;
            };
            if self.kind_name(node.kind) == "DataTypeOrVoid" {
                if let Some(ty) = self.type_from_children(&node.children) {
                    return Some(ty);
                }
                if let Some(first_child) = node.children.first() {
                    return Some(*first_child);
                }
            } else if let Some(ty) = self.type_from_children(&node.children) {
                return Some(ty);
            }
        }
        None
    }

    fn node(&self, id: u32) -> Option<&NodeRec> {
        self.nodes.get(id as usize).and_then(|n| n.as_ref())
    }

    fn kind_name(&self, kind: u16) -> &str {
        self.kind_table.get(kind as usize).map(|s| s.as_str()).unwrap_or("")
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
