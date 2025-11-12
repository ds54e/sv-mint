use crate::sv::model::SvParserCfg;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use sv_parser::{parse_sv_pp, parse_sv_str, preprocess, Define, DefineText, PreprocessedText, SyntaxTree};

pub(crate) type DefineMap = HashMap<String, Option<Define>>;

pub(crate) struct ParserInputs {
    include_paths: Vec<PathBuf>,
    pre_defines: DefineMap,
    strip_comments: bool,
    ignore_include: bool,
    allow_incomplete: bool,
}

pub(crate) struct PreprocessResult {
    pub text: String,
    pub block: Option<PreprocessedText>,
    pub defines: DefineMap,
}

pub(crate) struct ParseResult {
    pub syntax_tree: Option<SyntaxTree>,
    pub defines: DefineMap,
    pub has_cst: bool,
}

impl ParserInputs {
    pub fn new(cfg: &SvParserCfg) -> Self {
        let include_paths = cfg.include_paths.iter().map(PathBuf::from).collect();
        let mut pre_defines: DefineMap = HashMap::new();
        for raw in &cfg.defines {
            if let Some((name, def)) = parse_define_entry(raw) {
                pre_defines.insert(name, def);
            }
        }
        Self {
            include_paths,
            pre_defines,
            strip_comments: cfg.strip_comments,
            ignore_include: cfg.ignore_include,
            allow_incomplete: cfg.allow_incomplete,
        }
    }

    pub fn preprocess(&self, path: &Path, raw_text: &str) -> PreprocessResult {
        match preprocess(
            path,
            &self.pre_defines,
            &self.include_paths,
            self.strip_comments,
            self.ignore_include,
        ) {
            Ok((block, defines)) => PreprocessResult {
                text: block.text().to_owned(),
                block: Some(block),
                defines,
            },
            Err(_) => PreprocessResult {
                text: raw_text.to_owned(),
                block: None,
                defines: self.pre_defines.clone(),
            },
        }
    }

    pub fn parse(&self, path: &Path, result: PreprocessResult) -> ParseResult {
        let PreprocessResult { text, block, defines } = result;
        if let Some(block) = block {
            match parse_sv_pp(block, defines, self.allow_incomplete) {
                Ok((syntax_tree, defines)) => ParseResult {
                    syntax_tree: Some(syntax_tree),
                    defines,
                    has_cst: true,
                },
                Err(_) => ParseResult {
                    syntax_tree: None,
                    defines: self.pre_defines.clone(),
                    has_cst: false,
                },
            }
        } else {
            match parse_sv_str(
                &text,
                path,
                &self.pre_defines,
                &self.include_paths,
                self.ignore_include,
                self.allow_incomplete,
            ) {
                Ok((syntax_tree, defines)) => ParseResult {
                    syntax_tree: Some(syntax_tree),
                    defines,
                    has_cst: true,
                },
                Err(_) => ParseResult {
                    syntax_tree: None,
                    defines: self.pre_defines.clone(),
                    has_cst: false,
                },
            }
        }
    }
}

fn parse_define_entry(raw: &str) -> Option<(String, Option<Define>)> {
    let mut parts = raw.splitn(2, '=');
    let name = parts.next()?.trim();
    if name.is_empty() {
        return None;
    }
    if let Some(rest) = parts.next() {
        let define_text = DefineText::new(rest.trim().to_string(), None);
        let define = Define::new(name.to_string(), Vec::new(), Some(define_text));
        Some((name.to_string(), Some(define)))
    } else {
        Some((name.to_string(), None))
    }
}
