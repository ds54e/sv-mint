use crate::core::linemap::{LineMap, SpanBytes};
use crate::diag::event::{Ev, Event};
use crate::diag::logging::log_event;
use crate::sv::model::{AstSummary, DefineInfo, ParseArtifacts};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;
use std::path::Path;
use std::time::Instant;
use sv_parser::{parse_sv, preprocess, unwrap_node, Locate, RefNode, SyntaxTree};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct SvParserCfg {
    pub include_paths: Vec<String>,
    pub defines: Vec<String>,
    pub strip_comments: bool,
    pub ignore_include: bool,
    pub allow_incomplete: bool,
}

pub struct SvDriver<'a> {
    cfg: &'a SvParserCfg,
}

impl<'a> SvDriver<'a> {
    pub fn new(cfg: &'a SvParserCfg) -> Self {
        Self { cfg }
    }

    pub fn parse_text(&self, text: &str, input_path: &Path) -> ParseArtifacts {
        let path_s = input_path.to_string_lossy().into_owned();
        let raw_text = text.to_owned();
        let line_map = LineMap::new(&raw_text);

        let include_paths: Vec<std::path::PathBuf> =
            self.cfg.include_paths.iter().map(std::path::PathBuf::from).collect();

        type Defines = HashMap<String, Option<sv_parser::Define>>;
        let mut pre_defines: Defines = HashMap::new();
        for d in &self.cfg.defines {
            if let Some(eq) = d.find('=') {
                let name = d[..eq].to_string();
                pre_defines.insert(name, None);
            } else {
                pre_defines.insert(d.clone(), None);
            }
        }

        log_event(Ev::new(Event::ParsePreprocessStart, &path_s));
        let t0 = Instant::now();
        let pp_text = match preprocess(
            input_path,
            &pre_defines,
            &include_paths,
            self.cfg.strip_comments,
            self.cfg.ignore_include,
        ) {
            Ok((pp, _defs)) => pp.text().to_owned(),
            Err(_) => raw_text.clone(),
        };
        let elapsed_pp = t0.elapsed().as_millis();
        log_event(Ev::new(Event::ParsePreprocessDone, &path_s).with_duration_ms(elapsed_pp));

        log_event(Ev::new(Event::ParseParseStart, &path_s));
        let t1 = Instant::now();
        let (has_cst, final_defines, decls, refs, symbols) = match parse_sv(
            input_path,
            &pre_defines,
            &include_paths,
            self.cfg.ignore_include,
            self.cfg.allow_incomplete,
        ) {
            Ok((syntax_tree, defs)) => {
                let d = collect_decls_with_loc(&syntax_tree, &line_map);
                let r = collect_refs_with_modules(&syntax_tree);
                let s = analyze_symbols(&d, &r);
                (true, defs, d, r, s)
            }
            Err(_) => (false, pre_defines.clone(), Vec::new(), Vec::new(), Vec::new()),
        };
        let elapsed_parse = t1.elapsed().as_millis();
        log_event(Ev::new(Event::ParseParseDone, &path_s).with_duration_ms(elapsed_parse));
        log_event(Ev::new(Event::ParseAstCollectDone, &path_s));

        let defines: Vec<DefineInfo> = defines_to_info(&final_defines);
        let ast = AstSummary { decls, refs, symbols };

        ParseArtifacts {
            raw_text,
            pp_text,
            defines,
            has_cst,
            ast,
            line_map,
        }
    }
}

fn defines_to_info(defs: &HashMap<String, Option<sv_parser::Define>>) -> Vec<DefineInfo> {
    let mut v = Vec::with_capacity(defs.len());
    for (name, opt_def) in defs.iter() {
        let val = opt_def.as_ref().and_then(|d| d.text.as_ref().map(|t| t.text.clone()));
        v.push(DefineInfo {
            name: name.clone(),
            value: val,
        });
    }
    v
}

fn to_loc_json(st: &SyntaxTree, lm: &LineMap, idloc: &Locate) -> serde_json::Value {
    if let Some((_p, start)) = st.get_origin(idloc) {
        if let Some(s) = st.get_str(idloc) {
            let end = start + s.len();
            let span = SpanBytes { start, end };
            let ln = lm.to_lines(span);
            return json!({
                "line": ln.line,
                "col": ln.col,
                "end_line": ln.end_line,
                "end_col": ln.end_col
            });
        }
    }
    json!({
        "line": 1, "col": 1, "end_line": 1, "end_col": 1
    })
}

fn collect_decls_with_loc(syntax_tree: &SyntaxTree, line_map: &LineMap) -> Vec<serde_json::Value> {
    let mut decls = Vec::new();
    let mut current_module: Option<String> = None;
    for node in syntax_tree {
        if let RefNode::ModuleDeclarationNonansi(x) = node {
            if let Some(id) = unwrap_node!(x, ModuleIdentifier) {
                if let Some(idloc) = get_identifier(id) {
                    if let Some(name) = syntax_tree.get_str(&idloc) {
                        let loc = to_loc_json(syntax_tree, line_map, &idloc);
                        current_module = Some(name.to_string());
                        decls.push(json!({ "kind":"module", "name":name, "loc": loc }));
                    }
                }
            }
            continue;
        }
        if let RefNode::ModuleDeclarationAnsi(x) = node {
            if let Some(id) = unwrap_node!(x, ModuleIdentifier) {
                if let Some(idloc) = get_identifier(id) {
                    if let Some(name) = syntax_tree.get_str(&idloc) {
                        let loc = to_loc_json(syntax_tree, line_map, &idloc);
                        current_module = Some(name.to_string());
                        decls.push(json!({ "kind":"module", "name":name, "loc": loc }));
                    }
                }
            }
            continue;
        }
        if let RefNode::ParamAssignment(x) = node {
            let rn: RefNode = RefNode::from(x);
            if let Some(idloc) = get_identifier(rn) {
                if let Some(name) = syntax_tree.get_str(&idloc) {
                    let loc = to_loc_json(syntax_tree, line_map, &idloc);
                    decls.push(json!({
                        "kind":"param",
                        "name":name,
                        "module": current_module.clone().unwrap_or_default(),
                        "loc": loc
                    }));
                }
            }
            continue;
        }
        if let RefNode::NetDeclAssignment(x) = node {
            let rn: RefNode = RefNode::from(x);
            if let Some(idloc) = get_identifier(rn) {
                if let Some(name) = syntax_tree.get_str(&idloc) {
                    let loc = to_loc_json(syntax_tree, line_map, &idloc);
                    decls.push(json!({
                        "kind":"net",
                        "name":name,
                        "module": current_module.clone().unwrap_or_default(),
                        "loc": loc
                    }));
                }
            }
            continue;
        }
        if let RefNode::VariableDeclAssignment(x) = node {
            let rn: RefNode = RefNode::from(x);
            if let Some(idloc) = get_identifier(rn) {
                if let Some(name) = syntax_tree.get_str(&idloc) {
                    let loc = to_loc_json(syntax_tree, line_map, &idloc);
                    decls.push(json!({
                        "kind":"var",
                        "name":name,
                        "module": current_module.clone().unwrap_or_default(),
                        "loc": loc
                    }));
                }
            }
            continue;
        }
    }
    decls
}

fn collect_refs_with_modules(syntax_tree: &SyntaxTree) -> Vec<serde_json::Value> {
    let mut refs = Vec::new();
    let mut current_module: Option<String> = None;
    for node in syntax_tree {
        if let RefNode::ModuleDeclarationNonansi(x) = node {
            if let Some(id) = unwrap_node!(x, ModuleIdentifier) {
                if let Some(idloc) = get_identifier(id) {
                    if let Some(name) = syntax_tree.get_str(&idloc) {
                        current_module = Some(name.to_string());
                    }
                }
            }
            continue;
        }
        if let RefNode::ModuleDeclarationAnsi(x) = node {
            if let Some(id) = unwrap_node!(x, ModuleIdentifier) {
                if let Some(idloc) = get_identifier(id) {
                    if let Some(name) = syntax_tree.get_str(&idloc) {
                        current_module = Some(name.to_string());
                    }
                }
            }
            continue;
        }
        if let RefNode::HierarchicalIdentifier(x) = node {
            let rn: RefNode = RefNode::from(x);
            if let Some(idloc) = get_identifier(rn) {
                if let Some(name) = syntax_tree.get_str(&idloc) {
                    refs.push(json!({
                        "name": name,
                        "module": current_module.clone().unwrap_or_default()
                    }));
                }
            }
            continue;
        }
    }
    refs
}

fn analyze_symbols(decls: &[serde_json::Value], refs: &[serde_json::Value]) -> Vec<serde_json::Value> {
    let mut decls_map: HashMap<(String, String, String), serde_json::Value> = HashMap::new();
    for d in decls {
        let kind = d.get("kind").and_then(|x| x.as_str()).unwrap_or("");
        if kind == "param" || kind == "net" || kind == "var" {
            let name = d.get("name").and_then(|x| x.as_str()).unwrap_or("").to_string();
            let module = d.get("module").and_then(|x| x.as_str()).unwrap_or("").to_string();
            let loc = d.get("loc").cloned().unwrap_or_else(|| {
                json!({
                    "line": 1, "col": 1, "end_line": 1, "end_col": 1
                })
            });
            decls_map.insert((module, name, kind.to_string()), loc);
        }
    }

    let mut ref_counts: HashMap<(String, String), usize> = HashMap::new();
    for r in refs {
        let name = r.get("name").and_then(|x| x.as_str()).unwrap_or("").to_string();
        let module = r.get("module").and_then(|x| x.as_str()).unwrap_or("").to_string();
        *ref_counts.entry((module, name)).or_insert(0) += 1;
    }

    let mut out = Vec::with_capacity(decls_map.len());
    for ((module, name, class), loc) in decls_map {
        let n = *ref_counts.get(&(module.clone(), name.clone())).unwrap_or(&0);
        out.push(json!({
            "module": module,
            "name": name,
            "class": class,
            "ref_count": n,
            "used": n > 0,
            "loc": loc
        }));
    }
    out
}

fn get_identifier(node: RefNode) -> Option<Locate> {
    match unwrap_node!(node, SimpleIdentifier, EscapedIdentifier) {
        Some(RefNode::SimpleIdentifier(x)) => Some(x.nodes.0),
        Some(RefNode::EscapedIdentifier(x)) => Some(x.nodes.0),
        _ => None,
    }
}

trait EvExt<'a> {
    fn with_duration_ms(self, ms: u128) -> Self;
}

impl<'a> EvExt<'a> for Ev<'a> {
    fn with_duration_ms(mut self, ms: u128) -> Self {
        self.duration_ms = Some(ms);
        self
    }
}
