use crate::config::Config;
use crate::errors::ParseError;
use crate::textutil::{line_starts, linecol_at};
use serde::Deserialize;
use serde_json::{json, Value};
use std::collections::{BTreeMap, HashMap};
use std::path::{Path, PathBuf};
use sv_parser::{
    parse_sv, preprocess, unwrap_locate, unwrap_node, Define, DefineText, Defines, Locate, NodeEvent, RefNode,
    SyntaxTree,
};

#[derive(Debug, Clone, Deserialize, Default)]
pub struct SvParserCfg {
    pub include_paths: Vec<String>,
    pub defines: Vec<String>,
    pub strip_comments: bool,
    pub ignore_include: bool,
    pub allow_incomplete: bool,
}

pub struct FinalDefs {
    pub names: Vec<String>,
}

#[derive(Clone, Copy, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
pub enum DeclType {
    Net,
    Var,
    Param,
    Typedef,
    Other,
}
#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct DeclInfo {
    pub name: String,
    pub kind: String,
    pub decl_type: DeclType,
    pub data_type: Option<String>,
    pub range: Option<(i64, i64)>,
    pub init: Option<String>,
    pub file: String,
    pub line: u32,
    pub col: u32,
    pub scope: Vec<String>,
    #[serde(skip_serializing)]
    pub byte_begin: usize,
}

#[derive(Clone, Copy, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
pub enum RefKind {
    Lhs,
    Rhs,
    PortConn,
    TypeRef,
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct ReferenceInfo {
    pub name: String,
    pub kind: RefKind,
    pub file: String,
    pub line: u32,
    pub col: u32,
    pub scope: Vec<String>,
    #[serde(skip_serializing)]
    pub byte_begin: usize,
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct SymbolInfo {
    pub decl: DeclInfo,
    pub refs: Vec<usize>,
    pub rw_class: String,
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct SymbolTable {
    pub scopes: BTreeMap<String, BTreeMap<String, SymbolInfo>>,
}

pub fn run_svparser(
    input_path: &Path,
    cfg_dir: &Path,
    opt: &SvParserCfg,
) -> Result<(String, FinalDefs, Option<SyntaxTree>), ParseError> {
    let abs_includes = absolutize_many(cfg_dir, &opt.include_paths);
    let pre = build_predefines(&opt.defines).map_err(|e| ParseError::PreprocessFailed { detail: e })?;
    let (pp_text_pre, final_defs) = preprocess(input_path, &pre, &abs_includes, opt.strip_comments, opt.ignore_include)
        .map_err(|e| ParseError::PreprocessFailed {
            detail: format!("{}", e),
        })?;
    let tree = parse_sv(
        input_path,
        &pre,
        &abs_includes,
        opt.ignore_include,
        opt.allow_incomplete,
    )
    .map(|(t, _)| Some(t))
    .map_err(|e| ParseError::ParseFailed {
        detail: format!("{}", e),
    })?;
    let names = collect_define_names(&final_defs);
    Ok((pp_text_pre.text().to_string(), FinalDefs { names }, tree))
}

pub fn build_pp_payload(cfg: &Config, pp_text: &str, final_defs: &FinalDefs) -> Value {
    let predefined_names = cfg
        .svparser
        .defines
        .iter()
        .map(|s| s.split_once('=').map(|(n, _)| n).unwrap_or(s.as_str()).to_string())
        .collect::<Vec<_>>();
    let rules_block: Value = cfg.rules.clone();
    json!({
        "text": pp_text,
        "include_paths": cfg.svparser.include_paths,
        "defines": cfg.svparser.defines,
        "defines_table": final_defs.names,
        "defines_table_meta": [],
        "predefined_names": predefined_names,
        "rules": rules_block
    })
}

pub fn build_cst_payload(_cst_opt: &Option<SyntaxTree>) -> Value {
    json!({ "cst": null })
}

fn collect_declarations_and_references(tree: &SyntaxTree, file: &str) -> (Vec<DeclInfo>, Vec<ReferenceInfo>) {
    let mut declarations: Vec<DeclInfo> = Vec::new();
    let mut references: Vec<ReferenceInfo> = Vec::new();

    let mut in_port_decl: bool = false;
    let mut port_decl_kind: Option<&'static str> = None;

    let mut in_data_decl = false;
    let mut in_data_type_head = false;

    let mut in_param_decl = false;
    let mut in_param_assign = false;

    let mut in_typedef = false;
    let mut typedef_last_name: Option<String> = None;
    let mut typedef_last_byte: Option<usize> = None;

    let mut lhs_depth: i32 = 0;

    let scope: Vec<String> = Vec::new();
    let mut decl_seen: std::collections::HashSet<String> = std::collections::HashSet::new();
    let str_of = |tree: &SyntaxTree, loc: &Locate| -> String { tree.get_str(loc).unwrap().to_string() };

    for ev in tree.into_iter().event() {
        match ev {
            NodeEvent::Enter(n) => match n {
                RefNode::AnsiPortDeclaration(_) | RefNode::PortDeclaration(_) | RefNode::ListOfPortDeclarations(_) => {
                    in_port_decl = true;
                }
                RefNode::InputDeclaration(_) => {
                    port_decl_kind = Some("input");
                }
                RefNode::OutputDeclaration(_) => {
                    port_decl_kind = Some("output");
                }
                RefNode::InoutDeclaration(_) => {
                    port_decl_kind = Some("inout");
                }

                RefNode::DataDeclaration(_) => {
                    in_data_decl = true;
                    in_data_type_head = true;
                }

                RefNode::VariableLvalue(_) | RefNode::NetLvalue(_) => {
                    lhs_depth += 1;
                }

                RefNode::PortIdentifier(_) => {
                    if in_port_decl || port_decl_kind.is_some() {
                        if let Some(loc) = unwrap_locate!(n) {
                            let name = str_of(tree, loc);
                            if decl_seen.insert(name.clone()) {
                                let k = port_decl_kind.unwrap_or("port").to_string();
                                declarations.push(DeclInfo {
                                    name,
                                    kind: k,
                                    decl_type: DeclType::Var,
                                    data_type: None,
                                    range: None,
                                    init: None,
                                    file: file.to_string(),
                                    line: 1,
                                    col: 1,
                                    scope: scope.clone(),
                                    byte_begin: loc.offset,
                                });
                            }
                        }
                    }
                }

                RefNode::VariableIdentifier(_) | RefNode::NetIdentifier(_) => {
                    if in_data_decl {
                        in_data_type_head = false;
                        if let Some(loc) = unwrap_locate!(n) {
                            let name = str_of(tree, loc);
                            if decl_seen.insert(name.clone()) {
                                declarations.push(DeclInfo {
                                    name,
                                    kind: "var".to_string(),
                                    decl_type: DeclType::Var,
                                    data_type: None,
                                    range: None,
                                    init: None,
                                    file: file.to_string(),
                                    line: 1,
                                    col: 1,
                                    scope: scope.clone(),
                                    byte_begin: loc.offset,
                                });
                            }
                        }
                    }
                }

                RefNode::ParameterDeclarationParam(_)
                | RefNode::LocalParameterDeclarationParam(_)
                | RefNode::ParameterPortDeclaration(_) => {
                    in_param_decl = true;
                }

                RefNode::ParamAssignment(_) => {
                    if in_param_decl {
                        in_param_assign = true;
                    }
                }

                RefNode::Identifier(_) => {
                    if in_port_decl {
                        if let Some(id) = unwrap_node!(n, SimpleIdentifier, EscapedIdentifier) {
                            if let Some(loc) = unwrap_locate!(id) {
                                let name = str_of(tree, loc);
                                if decl_seen.insert(name.clone()) {
                                    declarations.push(DeclInfo {
                                        name,
                                        kind: "port".to_string(),
                                        decl_type: DeclType::Var,
                                        data_type: None,
                                        range: None,
                                        init: None,
                                        file: file.to_string(),
                                        line: 1,
                                        col: 1,
                                        scope: Vec::new(),
                                        byte_begin: loc.offset,
                                    });
                                }
                            }
                        }
                    } else if let Some(id) = unwrap_node!(n, SimpleIdentifier, EscapedIdentifier) {
                        if let Some(loc) = unwrap_locate!(id) {
                            let name = str_of(tree, loc);
                            if in_param_assign {
                                if decl_seen.insert(name.clone()) {
                                    declarations.push(DeclInfo {
                                        name,
                                        kind: "parameter".to_string(),
                                        decl_type: DeclType::Param,
                                        data_type: None,
                                        range: None,
                                        init: None,
                                        file: file.to_string(),
                                        line: 1,
                                        col: 1,
                                        scope: scope.clone(),
                                        byte_begin: loc.offset,
                                    });
                                }
                            } else if in_typedef {
                                typedef_last_name = Some(name);
                                typedef_last_byte = Some(loc.offset);
                            } else if in_data_decl && in_data_type_head {
                                references.push(ReferenceInfo {
                                    name,
                                    kind: RefKind::TypeRef,
                                    file: file.to_string(),
                                    line: 1,
                                    col: 1,
                                    scope: scope.clone(),
                                    byte_begin: loc.offset,
                                });
                            } else if lhs_depth > 0 {
                                references.push(ReferenceInfo {
                                    name,
                                    kind: RefKind::Lhs,
                                    file: file.to_string(),
                                    line: 1,
                                    col: 1,
                                    scope: scope.clone(),
                                    byte_begin: loc.offset,
                                });
                            } else {
                                references.push(ReferenceInfo {
                                    name,
                                    kind: RefKind::Rhs,
                                    file: file.to_string(),
                                    line: 1,
                                    col: 1,
                                    scope: scope.clone(),
                                    byte_begin: loc.offset,
                                });
                            }
                        }
                    }
                }

                RefNode::TypeDeclaration(_) => {
                    in_typedef = true;
                    typedef_last_name = None;
                    typedef_last_byte = None;
                }

                _ => {}
            },
            NodeEvent::Leave(n) => match n {
                RefNode::AnsiPortDeclaration(_) | RefNode::PortDeclaration(_) | RefNode::ListOfPortDeclarations(_) => {
                    in_port_decl = false;
                }

                RefNode::InputDeclaration(_) | RefNode::OutputDeclaration(_) | RefNode::InoutDeclaration(_) => {
                    port_decl_kind = None;
                }

                RefNode::DataDeclaration(_) => {
                    in_data_decl = false;
                    in_data_type_head = false;
                }

                RefNode::VariableLvalue(_) | RefNode::NetLvalue(_) => {
                    lhs_depth -= 1;
                }

                RefNode::ParamAssignment(_) => {
                    in_param_assign = false;
                }

                RefNode::ParameterDeclarationParam(_)
                | RefNode::LocalParameterDeclarationParam(_)
                | RefNode::ParameterPortDeclaration(_) => {
                    in_param_decl = false;
                }

                RefNode::TypeDeclaration(_) => {
                    if let Some(name) = typedef_last_name.take() {
                        declarations.push(DeclInfo {
                            name,
                            kind: "typedef".to_string(),
                            decl_type: DeclType::Typedef,
                            data_type: None,
                            range: None,
                            init: None,
                            file: file.to_string(),
                            line: 1,
                            col: 1,
                            scope: scope.clone(),
                            byte_begin: typedef_last_byte.unwrap_or(0),
                        });
                    }
                    in_typedef = false;
                }

                _ => {}
            },
        }
    }

    (declarations, references)
}

fn build_symbol_table_and_rw_class(declarations: &[DeclInfo], references: &[ReferenceInfo]) -> SymbolTable {
    let mut scopes: BTreeMap<String, BTreeMap<String, SymbolInfo>> = BTreeMap::new();
    if !declarations.is_empty() {
        let mut index_by_name: HashMap<(String, String), usize> = HashMap::new();
        for (i, d) in declarations.iter().enumerate() {
            index_by_name.insert((scope_key(&d.scope), d.name.clone()), i);
        }
        let mut refs_indexed: Vec<Vec<usize>> = vec![Vec::new(); declarations.len()];
        for (i, r) in references.iter().enumerate() {
            if let Some(&di) = index_by_name.get(&(scope_key(&r.scope), r.name.clone())) {
                refs_indexed[di].push(i);
            }
        }
        for (di, d) in declarations.iter().enumerate() {
            let scope = scope_key(&d.scope);
            let name = d.name.clone();
            let rs = refs_indexed.get(di).cloned().unwrap_or_default();
            let mut has_l = false;
            let mut has_r = false;
            for &ri in &rs {
                let k = references[ri].kind;
                if k == RefKind::Lhs {
                    has_l = true;
                } else if k == RefKind::Rhs || k == RefKind::TypeRef {
                    has_r = true;
                }
            }
            let cls = if has_l && has_r {
                "read_write"
            } else if has_l {
                "write_only"
            } else if has_r {
                "read_only"
            } else {
                "unused"
            };
            scopes.entry(scope).or_default().insert(
                name,
                SymbolInfo {
                    decl: d.clone(),
                    refs: rs,
                    rw_class: cls.to_string(),
                },
            );
        }
    }
    SymbolTable { scopes }
}

pub fn build_ast_payload(input_path: &Path, pp_text: &str, cst_opt: &Option<SyntaxTree>) -> Value {
    let file = input_path.to_string_lossy().to_string();
    let mut declarations: Vec<DeclInfo> = Vec::new();
    let mut references: Vec<ReferenceInfo> = Vec::new();

    if let Some(tree) = cst_opt {
        let (decls, refs) = collect_declarations_and_references(tree, &file);
        declarations = decls;
        references = refs;
    }

    let starts = line_starts(pp_text);
    for d in &mut declarations {
        let (ln, col) = linecol_at(&starts, d.byte_begin);
        d.line = ln;
        d.col = col;
    }
    for r in &mut references {
        let (ln, col) = linecol_at(&starts, r.byte_begin);
        r.line = ln;
        r.col = col;
    }
    let symtab = build_symbol_table_and_rw_class(&declarations, &references);

    json!({
        "ast": {
            "declarations": declarations,
            "references": references,
            "symbol_table": symtab,
        }
    })
}

fn scope_key(scope: &[String]) -> String {
    if scope.is_empty() {
        "::".to_string()
    } else {
        scope.join("::")
    }
}

fn build_predefines(defines: &[String]) -> Result<HashMap<String, Option<Define>>, String> {
    let mut pre = HashMap::new();
    for d in defines {
        if let Some((n, v)) = d.split_once('=') {
            let body = DefineText::new(v.to_string(), None);
            pre.insert(n.to_string(), Some(Define::new(n.to_string(), Vec::new(), Some(body))));
        } else {
            pre.insert(d.to_string(), None);
        }
    }
    Ok(pre)
}

fn collect_define_names(defs: &Defines) -> Vec<String> {
    let mut v = defs.keys().cloned().collect::<Vec<_>>();
    v.sort();
    v
}

fn absolutize_many(base: &Path, rels: &[String]) -> Vec<PathBuf> {
    rels.iter().map(|p| base.join(p)).collect()
}
