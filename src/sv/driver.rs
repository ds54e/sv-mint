use crate::core::linemap::LineMap;
use crate::sv::model::{AstSummary, DefineInfo, ParseArtifacts};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct SvParserCfg {
    pub include_paths: Vec<String>,
    pub defines: Vec<String>,
    pub strip_comments: bool,
    pub ignore_include: bool,
    pub allow_incomplete: bool,
}

#[allow(dead_code)]
pub struct SvDriver<'a> {
    cfg: &'a SvParserCfg,
}

impl<'a> SvDriver<'a> {
    pub fn new(cfg: &'a SvParserCfg) -> Self {
        Self { cfg }
    }

    pub fn parse_text(&self, text: &str) -> ParseArtifacts {
        let raw_text = text.to_string();
        let pp_text = raw_text.clone();
        let defines: Vec<DefineInfo> = Vec::new();
        let has_cst = false;
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
