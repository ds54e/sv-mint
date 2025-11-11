use crate::core::linemap::LineMap;
use crate::diag::event::{Ev, Event};
use crate::diag::logging::log_event;
use crate::sv::model::{AstSummary, DefineInfo, ParseArtifacts};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;
use std::time::Instant;

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
        let (pp_text, defs_after_pp) = match sv_parser::preprocess(
            input_path,
            &pre_defines,
            &include_paths,
            self.cfg.strip_comments,
            self.cfg.ignore_include,
        ) {
            Ok((pp, defs)) => (pp.text().to_owned(), Some(defs)),
            Err(_) => (raw_text.clone(), None),
        };
        let elapsed_pp = t0.elapsed().as_millis();
        log_event(Ev::new(Event::ParsePreprocessDone, &path_s).with_duration_ms(elapsed_pp));
        log_event(Ev::new(Event::ParseParseStart, &path_s));
        let t1 = Instant::now();
        let has_cst = sv_parser::parse_sv(
            input_path,
            &pre_defines,
            &include_paths,
            self.cfg.ignore_include,
            self.cfg.allow_incomplete,
        )
        .is_ok();
        let elapsed_parse = t1.elapsed().as_millis();
        log_event(Ev::new(Event::ParseParseDone, &path_s).with_duration_ms(elapsed_parse));
        log_event(Ev::new(Event::ParseAstCollectDone, &path_s));
        let defines: Vec<DefineInfo> = if let Some(defs) = defs_after_pp {
            let mut names: Vec<String> = defs.keys().cloned().collect();
            names.sort();
            names.into_iter().map(|n| DefineInfo { name: n, value: None }).collect()
        } else {
            let mut names: Vec<String> = pre_defines.keys().cloned().collect();
            names.sort();
            names.into_iter().map(|n| DefineInfo { name: n, value: None }).collect()
        };
        let ast = AstSummary::default();
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

trait EvExt<'a> {
    fn with_duration_ms(self, ms: u128) -> Self;
}

impl<'a> EvExt<'a> for Ev<'a> {
    fn with_duration_ms(mut self, ms: u128) -> Self {
        self.duration_ms = Some(ms);
        self
    }
}
