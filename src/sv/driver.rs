use crate::core::linemap::LineMap;
use crate::diag::event::{Ev, Event};
use crate::diag::logging::log_event;
use crate::sv::model::{AstSummary, DefineInfo, ParseArtifacts};
use serde::{Deserialize, Serialize};
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
        log_event(Ev::new(Event::ParsePreprocessStart, &path_s));
        let t0 = Instant::now();
        let pp_text = if self.cfg.strip_comments {
            strip_comments(text)
        } else {
            text.to_string()
        };
        let defines = parse_defines(&self.cfg.defines);
        let pp_elapsed = t0.elapsed().as_millis();
        log_event(Ev::new(Event::ParsePreprocessDone, &path_s).with_duration_ms(pp_elapsed));

        log_event(Ev::new(Event::ParseParseStart, &path_s));
        let t1 = Instant::now();
        let has_cst = !pp_text.is_empty();
        let parse_elapsed = t1.elapsed().as_millis();
        log_event(Ev::new(Event::ParseParseDone, &path_s).with_duration_ms(parse_elapsed));

        let t2 = Instant::now();
        let ast = AstSummary::default();
        let ast_elapsed = t2.elapsed().as_millis();
        log_event(Ev::new(Event::ParseAstCollectDone, &path_s).with_duration_ms(ast_elapsed));

        let raw_text = text.to_string();
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

fn parse_defines(items: &[String]) -> Vec<DefineInfo> {
    let mut out = Vec::with_capacity(items.len());
    for s in items {
        if let Some(pos) = s.find('=') {
            let name = s[..pos].to_string();
            let value = Some(s[pos + 1..].to_string());
            out.push(DefineInfo { name, value });
        } else {
            out.push(DefineInfo {
                name: s.to_string(),
                value: None,
            });
        }
    }
    out
}

fn strip_comments(src: &str) -> String {
    let mut out = String::with_capacity(src.len());
    let mut i = 0;
    let b = src.as_bytes();
    let n = b.len();
    while i < n {
        if i + 1 < n && b[i] == b'/' && b[i + 1] == b'/' {
            while i < n && b[i] != b'\n' {
                i += 1;
            }
        } else if i + 1 < n && b[i] == b'/' && b[i + 1] == b'*' {
            i += 2;
            while i + 1 < n && !(b[i] == b'*' && b[i + 1] == b'/') {
                i += 1;
            }
            if i + 1 < n {
                i += 2;
            }
        } else {
            out.push(b[i] as char);
            i += 1;
        }
    }
    out
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
