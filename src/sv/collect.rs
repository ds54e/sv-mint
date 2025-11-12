use crate::core::linemap::{LineMap, SpanBytes};
use crate::sv::model::{
    AssignOp, Assignment, DeclKind, Declaration, Reference, ReferenceKind, SymbolClass, SymbolUsage,
};
use crate::types::Location;
use std::collections::{HashMap, HashSet};
use sv_parser::{unwrap_node, Locate, NodeEvent, RefNode, SyntaxTree};

pub(crate) struct CollectResult {
    pub decls: Vec<Declaration>,
    pub refs: Vec<Reference>,
    pub assigns: Vec<Assignment>,
}

pub(crate) trait SyntaxVisitor {
    fn enter(&mut self, node: RefNode<'_>);
    fn leave(&mut self, node: RefNode<'_>);
}

pub(crate) fn walk_syntax(tree: &SyntaxTree, visitor: &mut impl SyntaxVisitor) {
    for event in tree.into_iter().event() {
        match event {
            NodeEvent::Enter(node) => visitor.enter(node),
            NodeEvent::Leave(node) => visitor.leave(node),
        }
    }
}

pub(crate) fn collect_all(syntax_tree: &SyntaxTree, line_map: &LineMap, raw_text: &str) -> CollectResult {
    let mut collector = AstCollector::new(line_map, raw_text, syntax_tree);
    walk_syntax(syntax_tree, &mut collector);
    collector.finish()
}

pub(crate) fn analyze_symbols(decls: &[Declaration], refs: &[Reference]) -> Vec<SymbolUsage> {
    let mut decl_locs: HashMap<(Option<String>, String, SymbolClass), Location> = HashMap::new();
    for d in decls {
        let class = match d.kind {
            DeclKind::Param => Some(SymbolClass::Param),
            DeclKind::Net => Some(SymbolClass::Net),
            DeclKind::Var => Some(SymbolClass::Var),
            _ => None,
        };
        if let Some(class) = class {
            decl_locs.insert((d.module.clone(), d.name.clone(), class), d.loc.clone());
        }
    }
    let mut read_counts: HashMap<(Option<String>, String), usize> = HashMap::new();
    let mut write_counts: HashMap<(Option<String>, String), usize> = HashMap::new();
    for r in refs {
        let key = (r.module.clone(), r.name.clone());
        match r.kind {
            ReferenceKind::Read => {
                *read_counts.entry(key).or_insert(0) += 1;
            }
            ReferenceKind::Write => {
                *write_counts.entry(key).or_insert(0) += 1;
            }
        }
    }
    let mut symbols = Vec::with_capacity(decl_locs.len());
    for ((module, name, class), loc) in decl_locs {
        let reads = *read_counts.get(&(module.clone(), name.clone())).unwrap_or(&0);
        let writes = *write_counts.get(&(module.clone(), name.clone())).unwrap_or(&0);
        let total = reads + writes;
        symbols.push(SymbolUsage {
            module,
            name,
            class,
            ref_count: total,
            read_count: reads,
            write_count: writes,
            used: total > 0,
            loc,
        });
    }
    symbols
}

struct AstCollector<'a> {
    line_map: &'a LineMap,
    raw_text: &'a str,
    syntax_tree: &'a SyntaxTree,
    scopes: Vec<String>,
    decls: Vec<Declaration>,
    refs: Vec<Reference>,
    assigns: Vec<Assignment>,
    write_offsets: HashSet<usize>,
    decl_offsets: HashSet<usize>,
}

impl<'a> AstCollector<'a> {
    fn new(line_map: &'a LineMap, raw_text: &'a str, syntax_tree: &'a SyntaxTree) -> Self {
        Self {
            line_map,
            raw_text,
            syntax_tree,
            scopes: Vec::new(),
            decls: Vec::new(),
            refs: Vec::new(),
            assigns: Vec::new(),
            write_offsets: HashSet::new(),
            decl_offsets: HashSet::new(),
        }
    }

    fn finish(self) -> CollectResult {
        CollectResult {
            decls: self.decls,
            refs: self.refs,
            assigns: self.assigns,
        }
    }

    fn module_info(&self, node: RefNode<'_>) -> Option<(String, Location)> {
        if let Some(id) = unwrap_node!(node, ModuleIdentifier) {
            if let Some(idloc) = get_identifier(id) {
                if let Some(name) = self.syntax_tree.get_str(&idloc) {
                    if let Some((loc, _)) = self.locate(&idloc) {
                        return Some((name.to_string(), loc));
                    }
                }
            }
        }
        None
    }

    fn record_decl(&mut self, node: RefNode<'_>, kind: DeclKind) {
        if let Some((ident, loc, origin)) = self.lookup_identifier(node) {
            let module = self.scopes.last().cloned();
            self.decls.push(Declaration {
                kind,
                name: ident,
                module,
                loc,
            });
            self.decl_offsets.insert(origin);
        }
    }

    fn record_write(&mut self, node: RefNode<'_>) {
        if let Some((ident, loc, origin)) = self.lookup_identifier(node) {
            let module = self.scopes.last().cloned();
            self.refs.push(Reference {
                name: ident.clone(),
                module: module.clone(),
                kind: ReferenceKind::Write,
                loc,
            });
            self.write_offsets.insert(origin);
            if let Some((op, lhs, rhs, start, end)) = scan_assignment_at(self.raw_text, origin) {
                let loc = self.span_location(start, end);
                self.assigns.push(Assignment {
                    module,
                    op,
                    lhs,
                    rhs,
                    loc,
                });
            }
        }
    }

    fn record_read(&mut self, node: RefNode<'_>) {
        if let Some((ident, loc, origin)) = self.lookup_identifier(node) {
            if self.write_offsets.contains(&origin) || self.decl_offsets.contains(&origin) {
                return;
            }
            let module = self.scopes.last().cloned();
            self.refs.push(Reference {
                name: ident,
                module,
                kind: ReferenceKind::Read,
                loc,
            });
        }
    }

    fn lookup_identifier(&self, node: RefNode<'_>) -> Option<(String, Location, usize)> {
        let idloc = get_identifier(node)?;
        let name = self.syntax_tree.get_str(&idloc)?.to_string();
        let (loc, origin) = self.locate(&idloc)?;
        Some((name, loc, origin))
    }

    fn locate(&self, idloc: &Locate) -> Option<(Location, usize)> {
        if let Some((_path, start)) = self.syntax_tree.get_origin(idloc) {
            if let Some(text) = self.syntax_tree.get_str(idloc) {
                let end = start + text.len();
                let span = SpanBytes::new(start, end);
                let lines = self.line_map.to_lines(span);
                let loc = Location {
                    line: lines.line,
                    col: lines.col,
                    end_line: lines.end_line,
                    end_col: lines.end_col,
                };
                return Some((loc, start));
            }
        }
        None
    }

    fn span_location(&self, start: usize, end: usize) -> Location {
        let span = SpanBytes::new(start, end);
        let lines = self.line_map.to_lines(span);
        Location {
            line: lines.line,
            col: lines.col,
            end_line: lines.end_line,
            end_col: lines.end_col,
        }
    }
}

impl<'a> SyntaxVisitor for AstCollector<'a> {
    fn enter(&mut self, node: RefNode<'_>) {
        match node {
            RefNode::ModuleDeclarationAnsi(x) => {
                if let Some((name, loc)) = self.module_info(RefNode::ModuleDeclarationAnsi(x)) {
                    self.decls.push(Declaration {
                        kind: DeclKind::Module,
                        name: name.clone(),
                        module: None,
                        loc,
                    });
                    self.scopes.push(name);
                }
            }
            RefNode::ModuleDeclarationNonansi(x) => {
                if let Some((name, loc)) = self.module_info(RefNode::ModuleDeclarationNonansi(x)) {
                    self.decls.push(Declaration {
                        kind: DeclKind::Module,
                        name: name.clone(),
                        module: None,
                        loc,
                    });
                    self.scopes.push(name);
                }
            }
            RefNode::ParamAssignment(x) => {
                self.record_decl(RefNode::from(x), DeclKind::Param);
            }
            RefNode::NetDeclAssignment(x) => {
                self.record_decl(RefNode::from(x), DeclKind::Net);
            }
            RefNode::VariableDeclAssignment(x) => {
                self.record_decl(RefNode::from(x), DeclKind::Var);
            }
            RefNode::NetLvalue(x) => {
                self.record_write(RefNode::from(x));
            }
            RefNode::VariableLvalue(x) => {
                self.record_write(RefNode::from(x));
            }
            RefNode::HierarchicalIdentifier(x) => {
                self.record_read(RefNode::from(x));
            }
            RefNode::SimpleIdentifier(x) => {
                self.record_read(RefNode::from(x));
            }
            _ => {}
        }
    }

    fn leave(&mut self, node: RefNode<'_>) {
        match node {
            RefNode::ModuleDeclarationAnsi(_) | RefNode::ModuleDeclarationNonansi(_) => {
                self.scopes.pop();
            }
            _ => {}
        }
    }
}

fn get_identifier(node: RefNode) -> Option<Locate> {
    match unwrap_node!(node, SimpleIdentifier, EscapedIdentifier) {
        Some(RefNode::SimpleIdentifier(x)) => Some(x.nodes.0),
        Some(RefNode::EscapedIdentifier(x)) => Some(x.nodes.0),
        _ => None,
    }
}

fn scan_assignment_at(text: &str, lhs_start: usize) -> Option<(AssignOp, String, String, usize, usize)> {
    let bytes = text.as_bytes();
    let mut i = lhs_start;
    let mut depth_paren = 0usize;
    let mut depth_brack = 0usize;
    let mut depth_brace = 0usize;
    while i < bytes.len() {
        let c = bytes[i] as char;
        match c {
            '(' => depth_paren += 1,
            ')' => depth_paren = depth_paren.saturating_sub(1),
            '[' => depth_brack += 1,
            ']' => depth_brack = depth_brack.saturating_sub(1),
            '{' => depth_brace += 1,
            '}' => depth_brace = depth_brace.saturating_sub(1),
            '<' => {
                if i + 1 < bytes.len()
                    && bytes[i + 1] as char == '='
                    && depth_paren == 0
                    && depth_brack == 0
                    && depth_brace == 0
                {
                    let lhs = text[lhs_start..i].trim().to_string();
                    let (rhs_start, rhs_end) = find_rhs_range(text, i + 2)?;
                    let rhs = text[rhs_start..rhs_end].trim().to_string();
                    return Some((AssignOp::Nonblocking, lhs, rhs, rhs_start, rhs_end));
                }
            }
            '=' => {
                let prev = if i > 0 { bytes[i - 1] as char } else { '\0' };
                let next = if i + 1 < bytes.len() {
                    bytes[i + 1] as char
                } else {
                    '\0'
                };
                if prev != '=' && next != '=' && depth_paren == 0 && depth_brack == 0 && depth_brace == 0 {
                    let lhs = text[lhs_start..i].trim().to_string();
                    let (rhs_start, rhs_end) = find_rhs_range(text, i + 1)?;
                    let rhs = text[rhs_start..rhs_end].trim().to_string();
                    return Some((AssignOp::BlockingOrCont, lhs, rhs, rhs_start, rhs_end));
                }
            }
            ';' => break,
            _ => {}
        }
        i += 1;
    }
    None
}

fn find_rhs_range(text: &str, rhs_start: usize) -> Option<(usize, usize)> {
    let bytes = text.as_bytes();
    let mut i = rhs_start;
    let mut depth_paren = 0usize;
    let mut depth_brack = 0usize;
    let mut depth_brace = 0usize;
    while i < bytes.len() {
        let c = bytes[i] as char;
        match c {
            '(' => depth_paren += 1,
            ')' => depth_paren = depth_paren.saturating_sub(1),
            '[' => depth_brack += 1,
            ']' => depth_brack = depth_brack.saturating_sub(1),
            '{' => depth_brace += 1,
            '}' => depth_brace = depth_brace.saturating_sub(1),
            ';' => {
                if depth_paren == 0 && depth_brack == 0 && depth_brace == 0 {
                    return Some((rhs_start, i));
                }
            }
            _ => {}
        }
        i += 1;
    }
    None
}
