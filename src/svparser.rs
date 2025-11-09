use crate::config::Config;
use anyhow::{anyhow, Result};
use serde_json::{json, Value};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use sv_parser::{parse_sv, preprocess, Define, DefineText, Defines, SyntaxTree};

#[derive(Default, serde::Deserialize)]
pub struct SvParserCfg {
    #[serde(default)]
    pub include_paths: Vec<String>,
    #[serde(default)]
    pub defines: Vec<String>,
    #[serde(default)]
    pub strip_comments: bool,
    #[serde(default)]
    pub ignore_include: bool,
    #[serde(default)]
    pub allow_incomplete: bool,
}

pub fn run_svparser(
    input_path: &Path,
    cfg_dir: &Path,
    sp: &SvParserCfg,
    normalized_text: &str,
) -> Result<(String, Defines, Option<SyntaxTree>)> {
    let incs: Vec<String> = sp
        .include_paths
        .iter()
        .map(|p| make_abs(cfg_dir, p).display().to_string())
        .collect();

    let mut pre_defines: HashMap<String, Option<Define>> = HashMap::new();
    for d in &sp.defines {
        if let Some((n, v)) = d.split_once('=') {
            let body = DefineText::new(v.to_string(), None);
            pre_defines.insert(n.to_string(), Some(Define::new(n.to_string(), Vec::new(), Some(body))));
        } else {
            pre_defines.insert(d.to_string(), None);
        }
    }

    let (_pp, final_defs) = preprocess(input_path, &pre_defines, &incs, sp.strip_comments, sp.ignore_include)
        .map_err(|_| anyhow!("preprocess failed"))?;
    let (tree, _) = parse_sv(input_path, &pre_defines, &incs, sp.ignore_include, sp.allow_incomplete)
        .map_err(|_| anyhow!("parsing failed"))?;

    Ok((normalized_text.to_string(), final_defs, Some(tree)))
}

pub fn build_pp_payload(cfg: &Config, pp_text: &str, final_defs: &Defines) -> Value {
    let mut names: Vec<String> = final_defs.keys().cloned().collect();
    names.sort();
    let mut predefined: Vec<String> = cfg
        .svparser
        .defines
        .iter()
        .filter_map(|s| s.split_once('=').map(|(n, _)| n).or(Some(s.as_str())))
        .map(|s| s.to_string())
        .collect();
    predefined.sort();
    predefined.dedup();
    json!({
        "text": pp_text,
        "include_paths": cfg.svparser.include_paths,
        "defines": cfg.svparser.defines,
        "defines_table": names,
        "predefined_names": predefined,
        "rules": cfg.rules
    })
}

pub fn build_cst_payload(cst_opt: &Option<SyntaxTree>) -> Value {
    let cst_json = cst_opt
        .as_ref()
        .map(|_| {
            json!({
                "kind":"SyntaxTree",
                "range":{"line":1,"col":1,"end_line":1,"end_col":1},
                "children":[]
            })
        })
        .unwrap_or_else(|| {
            json!({
                "kind":"Empty",
                "range":{"line":1,"col":1,"end_line":1,"end_col":1},
                "children":[]
            })
        });
    json!({ "cst": cst_json })
}

fn make_abs(base: &Path, rel_or_abs: &str) -> PathBuf {
    let p = PathBuf::from(rel_or_abs);
    if p.is_absolute() {
        p
    } else {
        base.join(p)
    }
}
