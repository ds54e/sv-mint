use crate::config::{read_input, Config};
use crate::output::print_violations;
use crate::plugin::run_plugin_once;
use crate::sv::model::ParseArtifacts;
use crate::svparser::SvDriver;
use crate::types::{Stage, Violation};
use anyhow::Result;
use serde_json::json;
use std::path::{Path, PathBuf};

pub struct RunSummary {
    pub violations: usize,
    pub had_error: bool,
}

pub struct Pipeline<'a> {
    cfg: &'a Config,
}

impl<'a> Pipeline<'a> {
    pub fn new(cfg: &'a Config) -> Self {
        Self { cfg }
    }

    pub fn run_file(&self, input: &Path) -> Result<usize> {
        let (normalized_text, input_path) = read_input(input)?;
        let driver = SvDriver::new(&self.cfg.svparser);
        let artifacts = driver.parse_text(&normalized_text);
        let mut all: Vec<Violation> = Vec::new();

        for stage in &self.cfg.stages.enabled {
            let payload = payload_for(stage, &artifacts);
            let vs = run_plugin_once(stage.as_str(), &input_path, payload)?;
            all.extend(vs);
        }

        print_violations(&all, &input_path);
        Ok(all.len())
    }

    pub fn run_files(&self, inputs: &[PathBuf]) -> RunSummary {
        let mut had_error = false;
        let mut n_viol: usize = 0;

        for inp in inputs {
            match self.run_file(inp) {
                Ok(n) => n_viol += n,
                Err(_) => had_error = true,
            }
        }
        RunSummary {
            violations: n_viol,
            had_error,
        }
    }
}

fn payload_for(stage: &Stage, a: &ParseArtifacts) -> serde_json::Value {
    match stage {
        Stage::RawText => json!({ "text": a.raw_text }),
        Stage::PpText => json!({
            "text": a.pp_text,
            "defines": a.defines.iter().map(|d| json!({
                "name": d.name,
                "value": d.value
            })).collect::<Vec<_>>()
        }),
        Stage::Cst => json!({ "has_cst": a.has_cst }),
        Stage::Ast => json!({
            "decls": a.ast.decls,
            "refs": a.ast.refs,
            "symbols": a.ast.symbols
        }),
    }
}
