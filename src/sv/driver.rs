use crate::core::errors::ParseError;
use crate::core::linemap::LineMap;
use crate::diag::event::{Ev, Event};
use crate::diag::logging::log_event;
use crate::sv::collect::{analyze_symbols, collect_all};
use crate::sv::cst_ir::build_cst_ir;
pub use crate::sv::model::SvParserCfg;
use crate::sv::model::{AstSummary, DefineInfo, ParseArtifacts};
use crate::sv::preprocess::ParserInputs;
use std::path::Path;
use std::time::Instant;

pub struct SvDriver {
    inputs: ParserInputs,
}

impl SvDriver {
    pub fn new(cfg: &SvParserCfg) -> Self {
        Self {
            inputs: ParserInputs::new(cfg),
        }
    }

    pub fn parse_text(&self, text: &str, input_path: &Path) -> Result<ParseArtifacts, ParseError> {
        let path_s = input_path.to_string_lossy().into_owned();
        let raw_text = text.to_owned();
        let line_map = LineMap::new(&raw_text);

        log_event(Ev::new(Event::ParsePreprocessStart, &path_s));
        let t0 = Instant::now();
        let preprocess = self.inputs.preprocess(input_path, &raw_text);
        let pp_text = preprocess.text.clone();
        let elapsed_pp = t0.elapsed().as_millis();
        log_event(Ev::new(Event::ParsePreprocessDone, &path_s).with_duration_ms(elapsed_pp));

        log_event(Ev::new(Event::ParseParseStart, &path_s));
        let t1 = Instant::now();
        let parse_out = self.inputs.parse(input_path, preprocess);
        let elapsed_parse = t1.elapsed().as_millis();
        log_event(Ev::new(Event::ParseParseDone, &path_s).with_duration_ms(elapsed_parse));

        let tree = parse_out.syntax_tree.as_ref().ok_or_else(|| ParseError::ParseFailed {
            detail: format!("{}: parser produced no syntax tree", path_s),
        })?;
        let collect = collect_all(tree, &line_map, &raw_text);
        let has_cst = parse_out.has_cst;
        log_event(Ev::new(Event::ParseAstCollectDone, &path_s));

        let defines = defines_to_info(&parse_out.defines);
        let symbols = analyze_symbols(&collect.decls, &collect.refs);
        let ast = AstSummary {
            decls: collect.decls,
            refs: collect.refs,
            assigns: collect.assigns,
            symbols,
            pp_text: Some(pp_text.clone()),
            ..AstSummary::default()
        };

        let line_starts = line_starts(&pp_text);
        let cst_ir = if has_cst {
            Some(build_cst_ir(
                tree,
                &path_s,
                "",
                &line_starts,
                &pp_text,
            ))
        } else {
            None
        };

        Ok(ParseArtifacts {
            raw_text,
            pp_text,
            defines,
            has_cst,
            ast,
            cst_ir,
            line_map,
        })
    }
}

fn defines_to_info(defs: &crate::sv::preprocess::DefineMap) -> Vec<DefineInfo> {
    let mut out = Vec::with_capacity(defs.len());
    for (name, opt_def) in defs {
        let value = opt_def.as_ref().and_then(|d| d.text.as_ref()).map(|t| t.text.clone());
        out.push(DefineInfo {
            name: name.clone(),
            value,
        });
    }
    out
}

fn line_starts(s: &str) -> Vec<usize> {
    let mut starts = Vec::with_capacity(s.len() / 32 + 2);
    starts.push(0);
    for (i, ch) in s.char_indices() {
        if ch == '\n' {
            starts.push(i + 1);
        }
    }
    starts
}
