use crate::config::Config;
use anyhow::{anyhow, Result};
use serde::Deserialize;
use serde_json::{json, Value};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use sv_parser::{parse_sv, preprocess, Define, DefineText, Defines, SyntaxTree};

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

pub fn run_svparser(
    input_path: &Path,
    cfg_dir: &Path,
    opt: &SvParserCfg,
    normalized_text: &str,
) -> Result<(String, FinalDefs, Option<SyntaxTree>)> {
    let abs_includes = absolutize_many(cfg_dir, &opt.include_paths);
    let pre = build_predefines(&opt.defines)?;
    let (_pp_text_ignored, final_defs) =
        preprocess(input_path, &pre, &abs_includes, opt.strip_comments, opt.ignore_include)
            .map_err(|_| anyhow!("preprocess failed"))?;
    let tree = parse_sv(
        input_path,
        &pre,
        &abs_includes,
        opt.ignore_include,
        opt.allow_incomplete,
    )
    .map(|(t, _)| Some(t))
    .map_err(|_| anyhow!("parsing failed"))?;
    let names = collect_define_names(&final_defs);
    let pp_text = normalized_text.to_string();
    Ok((pp_text, FinalDefs { names }, tree))
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

fn build_predefines(defines: &[String]) -> Result<HashMap<String, Option<Define>>> {
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
