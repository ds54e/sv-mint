use crate::core::linemap::LineMap;
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
        let (has_cst, final_defines, decls, refs) = match parse_sv(
            input_path,
            &pre_defines,
            &include_paths,
            self.cfg.ignore_include,
            self.cfg.allow_incomplete,
        ) {
            Ok((syntax_tree, defs)) => {
                let d = collect_decls(&syntax_tree);
                let r = collect_refs(&syntax_tree);
                (true, defs, d, r)
            }
            Err(_) => (false, pre_defines.clone(), Vec::new(), Vec::new()),
        };
        let elapsed_parse = t1.elapsed().as_millis();
        log_event(Ev::new(Event::ParseParseDone, &path_s).with_duration_ms(elapsed_parse));
        log_event(Ev::new(Event::ParseAstCollectDone, &path_s));

        let defines: Vec<DefineInfo> = defines_to_info(&final_defines);
        let ast = AstSummary {
            decls,
            refs,
            symbols: Vec::new(),
        };
        let line_map = LineMap::new(&raw_text);

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

fn collect_decls(syntax_tree: &SyntaxTree) -> Vec<serde_json::Value> {
    let mut decls = Vec::new();
    for node in syntax_tree {
        match node {
            RefNode::ModuleDeclarationNonansi(x) => {
                if let Some(id) = unwrap_node!(x, ModuleIdentifier) {
                    if let Some(idloc) = get_identifier(id) {
                        if let Some(name) = syntax_tree.get_str(&idloc) {
                            decls.push(json!({"kind":"module","name":name}));
                        }
                    }
                }
            }
            RefNode::ModuleDeclarationAnsi(x) => {
                if let Some(id) = unwrap_node!(x, ModuleIdentifier) {
                    if let Some(idloc) = get_identifier(id) {
                        if let Some(name) = syntax_tree.get_str(&idloc) {
                            decls.push(json!({"kind":"module","name":name}));
                        }
                    }
                }
            }
            RefNode::ParamAssignment(x) => {
                let rn: RefNode = RefNode::from(x);
                if let Some(idloc) = get_identifier(rn) {
                    if let Some(name) = syntax_tree.get_str(&idloc) {
                        decls.push(json!({"kind":"param","name":name}));
                    }
                }
            }
            RefNode::NetDeclAssignment(x) => {
                let rn: RefNode = RefNode::from(x);
                if let Some(idloc) = get_identifier(rn) {
                    if let Some(name) = syntax_tree.get_str(&idloc) {
                        decls.push(json!({"kind":"net","name":name}));
                    }
                }
            }
            RefNode::VariableDeclAssignment(x) => {
                let rn: RefNode = RefNode::from(x);
                if let Some(idloc) = get_identifier(rn) {
                    if let Some(name) = syntax_tree.get_str(&idloc) {
                        decls.push(json!({"kind":"var","name":name}));
                    }
                }
            }
            _ => {}
        }
    }
    decls
}

fn collect_refs(syntax_tree: &SyntaxTree) -> Vec<serde_json::Value> {
    let mut refs = Vec::new();
    for node in syntax_tree {
        if let RefNode::HierarchicalIdentifier(x) = node {
            let rn: RefNode = RefNode::from(x);
            if let Some(idloc) = get_identifier(rn) {
                if let Some(name) = syntax_tree.get_str(&idloc) {
                    refs.push(json!({"name":name}));
                }
            }
        }
    }
    refs
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
