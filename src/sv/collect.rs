use crate::core::errors::ParseError;
use crate::core::linemap::SpanBytes;
use crate::sv::model::{
    AssignOp, Assignment, DeclKind, Declaration, PortInfo, Reference, ReferenceKind, SymbolClass, SymbolUsage,
};
use crate::sv::source::{SourceCache, SourceFile};
use crate::types::Location;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use sv_parser::{unwrap_node, Locate, NodeEvent, RefNode, SyntaxTree};

type IdentifierLookup = Option<(String, Location, usize, Arc<SourceFile>)>;

pub(crate) struct CollectResult {
    pub decls: Vec<Declaration>,
    pub refs: Vec<Reference>,
    pub assigns: Vec<Assignment>,
    pub ports: Vec<PortInfo>,
}

pub(crate) trait SyntaxVisitor {
    fn enter(&mut self, node: RefNode<'_>) -> Result<(), ParseError>;
    fn leave(&mut self, node: RefNode<'_>) -> Result<(), ParseError>;
}

pub(crate) fn walk_syntax(tree: &SyntaxTree, visitor: &mut impl SyntaxVisitor) -> Result<(), ParseError> {
    for event in tree.into_iter().event() {
        match event {
            NodeEvent::Enter(node) => visitor.enter(node)?,
            NodeEvent::Leave(node) => visitor.leave(node)?,
        }
    }
    Ok(())
}

pub(crate) fn collect_all(syntax_tree: &SyntaxTree, sources: &mut SourceCache) -> Result<CollectResult, ParseError> {
    let mut collector = AstCollector::new(sources, syntax_tree);
    walk_syntax(syntax_tree, &mut collector)?;
    Ok(collector.finish())
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
    sources: &'a mut SourceCache,
    syntax_tree: &'a SyntaxTree,
    scopes: Vec<String>,
    decls: Vec<Declaration>,
    refs: Vec<Reference>,
    assigns: Vec<Assignment>,
    ports: Vec<PortInfo>,
    read_offsets: HashSet<usize>,
    write_offsets: HashSet<usize>,
    decl_offsets: HashSet<usize>,
    port_dir_stack: Vec<&'static str>,
}

impl<'a> AstCollector<'a> {
    fn new(sources: &'a mut SourceCache, syntax_tree: &'a SyntaxTree) -> Self {
        Self {
            sources,
            syntax_tree,
            scopes: Vec::new(),
            decls: Vec::new(),
            refs: Vec::new(),
            assigns: Vec::new(),
            ports: Vec::new(),
            read_offsets: HashSet::new(),
            write_offsets: HashSet::new(),
            decl_offsets: HashSet::new(),
            port_dir_stack: Vec::new(),
        }
    }

    fn finish(self) -> CollectResult {
        CollectResult {
            decls: self.decls,
            refs: self.refs,
            assigns: self.assigns,
            ports: self.ports,
        }
    }

    fn module_info(&mut self, node: RefNode<'_>) -> Result<Option<(String, Location)>, ParseError> {
        if let Some(id) = unwrap_node!(node, ModuleIdentifier) {
            if let Some(idloc) = get_identifier(id) {
                if let Some(name) = self.syntax_tree.get_str(&idloc) {
                    if let Some((loc, _, _)) = self.locate(&idloc)? {
                        return Ok(Some((name.to_string(), loc)));
                    }
                }
            }
        }
        Ok(None)
    }

    fn record_decl(&mut self, node: RefNode<'_>, kind: DeclKind) -> Result<(), ParseError> {
        let is_decl_assign = matches!(node, RefNode::NetDeclAssignment(_) | RefNode::VariableDeclAssignment(_));
        if let Some((ident, loc, origin, source)) = self.lookup_identifier(node)? {
            let module = self.scopes.last().cloned();
            if is_decl_assign {
                if let Some((op, lhs, rhs, start, end)) = scan_assignment_at(&source.text, origin) {
                    let aloc = self.location_from_source(source.as_ref(), start, end);
                    self.refs.push(Reference {
                        name: ident.clone(),
                        module: module.clone(),
                        kind: ReferenceKind::Write,
                        loc: loc.clone(),
                    });
                    self.write_offsets.insert(origin);
                    self.assigns.push(Assignment {
                        module: module.clone(),
                        op,
                        lhs,
                        rhs,
                        loc: aloc,
                    });
                }
            }
            self.decls.push(Declaration {
                kind,
                name: ident,
                module,
                loc,
            });
            self.decl_offsets.insert(origin);
        }
        Ok(())
    }

    fn record_write(&mut self, node: RefNode<'_>) -> Result<(), ParseError> {
        if let Some((ident, loc, origin, source)) = self.lookup_identifier(node)? {
            let module = self.scopes.last().cloned();
            self.refs.push(Reference {
                name: ident.clone(),
                module: module.clone(),
                kind: ReferenceKind::Write,
                loc,
            });
            self.write_offsets.insert(origin);
            if let Some((op, lhs, rhs, start, end)) = scan_assignment_at(&source.text, origin) {
                let loc = self.location_from_source(source.as_ref(), start, end);
                self.assigns.push(Assignment {
                    module,
                    op,
                    lhs,
                    rhs,
                    loc,
                });
            }
        }
        Ok(())
    }

    fn record_read(&mut self, node: RefNode<'_>) -> Result<(), ParseError> {
        if let Some((ident, loc, origin, _)) = self.lookup_identifier(node)? {
            if self.write_offsets.contains(&origin)
                || self.decl_offsets.contains(&origin)
                || self.read_offsets.contains(&origin)
            {
                return Ok(());
            }
            let module = self.scopes.last().cloned();
            self.refs.push(Reference {
                name: ident,
                module,
                kind: ReferenceKind::Read,
                loc,
            });
            self.read_offsets.insert(origin);
        }
        Ok(())
    }

    fn record_port(&mut self, name: String, loc: Location, direction: &str) {
        let module = self.scopes.last().cloned();
        self.ports.push(PortInfo {
            module,
            name,
            direction: direction.to_string(),
            loc,
        });
    }

    fn record_port_identifier(&mut self, node: RefNode<'_>) -> Result<(), ParseError> {
        if let Some(dir) = self.port_dir_stack.last().copied() {
            if let Some((name, loc, origin, _)) = self.lookup_identifier(node)? {
                self.record_port(name, loc, dir);
                self.decl_offsets.insert(origin);
            }
        }
        Ok(())
    }

    fn lookup_identifier(&mut self, node: RefNode<'_>) -> Result<IdentifierLookup, ParseError> {
        let Some(idloc) = get_identifier(node) else {
            return Ok(None);
        };
        let Some(name) = self.syntax_tree.get_str(&idloc) else {
            return Ok(None);
        };
        let Some((loc, origin, source)) = self.locate(&idloc)? else {
            return Ok(None);
        };
        Ok(Some((name.to_string(), loc, origin, source)))
    }

    fn locate(&mut self, idloc: &Locate) -> Result<Option<(Location, usize, Arc<SourceFile>)>, ParseError> {
        if let Some((path, start)) = self.syntax_tree.get_origin(idloc) {
            let source = self.sources.get_or_load(path)?;
            let end = start + idloc.len;
            let loc = self.location_from_source(source.as_ref(), start, end);
            return Ok(Some((loc, start, source)));
        }
        Ok(None)
    }

    fn location_from_source(&self, source: &SourceFile, start: usize, end: usize) -> Location {
        let span = SpanBytes::new(start, end);
        let lines = source.line_map.to_lines(span);
        Location {
            line: lines.line,
            col: lines.col,
            end_line: lines.end_line,
            end_col: lines.end_col,
            file: Some(source.display.clone()),
        }
    }
}

impl<'a> SyntaxVisitor for AstCollector<'a> {
    fn enter(&mut self, node: RefNode<'_>) -> Result<(), ParseError> {
        match node {
            RefNode::ModuleDeclarationAnsi(x) => {
                if let Some((name, loc)) = self.module_info(RefNode::ModuleDeclarationAnsi(x))? {
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
                if let Some((name, loc)) = self.module_info(RefNode::ModuleDeclarationNonansi(x))? {
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
                self.record_decl(RefNode::from(x), DeclKind::Param)?;
            }
            RefNode::LocalParameterDeclaration(x) => {
                self.record_decl(RefNode::from(x), DeclKind::LocalParam)?;
            }
            RefNode::NetDeclAssignment(x) => {
                self.record_decl(RefNode::from(x), DeclKind::Net)?;
            }
            RefNode::VariableDeclAssignment(x) => {
                self.record_decl(RefNode::from(x), DeclKind::Var)?;
            }
            RefNode::PortDeclarationInput(_) => self.port_dir_stack.push("input"),
            RefNode::PortDeclarationOutput(_) => self.port_dir_stack.push("output"),
            RefNode::PortDeclarationInout(_) => self.port_dir_stack.push("inout"),
            RefNode::PortDeclarationRef(_) => self.port_dir_stack.push("ref"),
            RefNode::PortDeclarationInterface(_) => self.port_dir_stack.push("interface"),
            RefNode::AnsiPortDeclarationNet(x) => {
                self.handle_ansi_port_net(x)?;
            }
            RefNode::AnsiPortDeclarationVariable(x) => {
                self.handle_ansi_port_var(x)?;
            }
            RefNode::AnsiPortDeclarationParen(x) => {
                self.handle_ansi_port_paren(x)?;
            }
            RefNode::NamedPortConnectionIdentifier(x) => {
                self.handle_named_port_connection_identifier(x)?;
            }
            RefNode::PortIdentifier(x) => {
                self.record_port_identifier(RefNode::PortIdentifier(x))?;
            }
            RefNode::NetLvalue(x) => {
                self.record_write(RefNode::from(x))?;
            }
            RefNode::VariableLvalue(x) => {
                self.record_write(RefNode::from(x))?;
            }
            RefNode::HierarchicalIdentifier(x) => {
                self.record_read(RefNode::from(x))?;
            }
            RefNode::SimpleIdentifier(x) => {
                self.record_read(RefNode::from(x))?;
            }
            _ => {}
        }
        Ok(())
    }

    fn leave(&mut self, node: RefNode<'_>) -> Result<(), ParseError> {
        match node {
            RefNode::ModuleDeclarationAnsi(_) | RefNode::ModuleDeclarationNonansi(_) => {
                self.scopes.pop();
            }
            RefNode::PortDeclarationInput(_)
            | RefNode::PortDeclarationOutput(_)
            | RefNode::PortDeclarationInout(_)
            | RefNode::PortDeclarationRef(_)
            | RefNode::PortDeclarationInterface(_) => {
                self.port_dir_stack.pop();
            }
            _ => {}
        }
        Ok(())
    }
}

impl<'a> AstCollector<'a> {
    fn handle_ansi_port_net(&mut self, port: &sv_parser::AnsiPortDeclarationNet) -> Result<(), ParseError> {
        if let Some((name, loc, origin, _)) = self.lookup_identifier(RefNode::from(&port.nodes.1))? {
            let direction = net_header_direction(port.nodes.0.as_ref());
            self.record_port(name, loc, direction);
            self.decl_offsets.insert(origin);
        }
        Ok(())
    }

    fn handle_ansi_port_var(&mut self, port: &sv_parser::AnsiPortDeclarationVariable) -> Result<(), ParseError> {
        if let Some((name, loc, origin, _)) = self.lookup_identifier(RefNode::from(&port.nodes.1))? {
            let direction = variable_header_direction(port.nodes.0.as_ref());
            self.record_port(name, loc, direction);
            self.decl_offsets.insert(origin);
        }
        Ok(())
    }

    fn handle_ansi_port_paren(&mut self, port: &sv_parser::AnsiPortDeclarationParen) -> Result<(), ParseError> {
        if let Some((name, loc, origin, _)) = self.lookup_identifier(RefNode::from(&port.nodes.2))? {
            let direction = port
                .nodes
                .0
                .as_ref()
                .map(port_direction_to_str)
                .unwrap_or("unspecified");
            self.record_port(name, loc, direction);
            self.decl_offsets.insert(origin);
        }
        Ok(())
    }

    fn handle_named_port_connection_identifier(
        &mut self,
        port: &sv_parser::NamedPortConnectionIdentifier,
    ) -> Result<(), ParseError> {
        if port.nodes.3.is_none() {
            // Implicit `.foo` shorthand: no actual expression, so treat the port name as a read of
            // the same-named signal in the current scope.
            self.record_read(RefNode::from(&port.nodes.2))?;
        }
        Ok(())
    }
}

fn get_identifier(node: RefNode) -> Option<Locate> {
    match unwrap_node!(node, SimpleIdentifier, EscapedIdentifier) {
        Some(RefNode::SimpleIdentifier(x)) => Some(x.nodes.0),
        Some(RefNode::EscapedIdentifier(x)) => Some(x.nodes.0),
        _ => None,
    }
}

fn port_direction_to_str(dir: &sv_parser::PortDirection) -> &'static str {
    match dir {
        sv_parser::PortDirection::Input(_) => "input",
        sv_parser::PortDirection::Output(_) => "output",
        sv_parser::PortDirection::Inout(_) => "inout",
        sv_parser::PortDirection::Ref(_) => "ref",
    }
}

fn net_header_direction(header: Option<&sv_parser::NetPortHeaderOrInterfacePortHeader>) -> &'static str {
    if let Some(header) = header {
        match header {
            sv_parser::NetPortHeaderOrInterfacePortHeader::NetPortHeader(h) => {
                h.nodes.0.as_ref().map(port_direction_to_str).unwrap_or("unspecified")
            }
            sv_parser::NetPortHeaderOrInterfacePortHeader::InterfacePortHeader(_) => "interface",
        }
    } else {
        "unspecified"
    }
}

fn variable_header_direction(header: Option<&sv_parser::VariablePortHeader>) -> &'static str {
    header
        .and_then(|h| h.nodes.0.as_ref().map(port_direction_to_str))
        .unwrap_or("unspecified")
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
