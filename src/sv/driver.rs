use serde::{Deserialize, Serialize};

use crate::sv::model::{AstSummary, DefineInfo, LineMap, ParseArtifacts};

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

        // M3段階では外部依存を導入せず、最小の疑似処理を行います。
        // 将来M4以降でsv-parserを組み込む場合、ここを差し替えます。
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
