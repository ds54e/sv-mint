use crate::core::errors::ParseError;
use crate::diag::event::{Ev, Event};
use crate::diag::logging::log_event;
use crate::sv::collect::{analyze_symbols, collect_all};
use crate::sv::cst_ir::build_cst_ir;
pub use crate::sv::model::SvParserCfg;
use crate::sv::model::{AstSummary, DefineInfo, ParseArtifacts};
use crate::sv::preprocess::ParserInputs;
use crate::sv::source::SourceCache;
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

    pub fn parse_text(
        &self,
        raw_text: &str,
        normalized_text: &str,
        input_path: &Path,
    ) -> Result<ParseArtifacts, ParseError> {
        let path_s = input_path.to_string_lossy().into_owned();
        let raw_owned = raw_text.to_owned();
        let mut sources = SourceCache::new(input_path, raw_owned.clone());

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
        let collect = collect_all(tree, &mut sources)?;
        let has_cst = parse_out.has_cst;
        log_event(Ev::new(Event::ParseAstCollectDone, &path_s));

        let defines = defines_to_info(&parse_out.defines);
        let symbols = analyze_symbols(&collect.decls, &collect.refs);
        let ast = AstSummary {
            decls: collect.decls,
            refs: collect.refs,
            assigns: collect.assigns,
            ports: collect.ports,
            symbols,
            pp_text: Some(pp_text.clone()),
            ..AstSummary::default()
        };

        let line_starts = line_starts(&pp_text);
        let cst_ir = if has_cst {
            Some(build_cst_ir(tree, &path_s, "", &line_starts, &pp_text))
        } else {
            None
        };

        Ok(ParseArtifacts {
            raw_text: raw_owned,
            normalized_text: normalized_text.to_owned(),
            pp_text,
            defines,
            has_cst,
            ast,
            cst_ir,
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
