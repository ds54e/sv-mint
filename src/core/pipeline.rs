use crate::config::{read_input, Config};
use crate::diag::event::{Ev, Event};
use crate::diag::logging::log_event;
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
    pub cfg: &'a Config,
}

impl<'a> Pipeline<'a> {
    pub fn new(cfg: &'a Config) -> Self {
        Self { cfg }
    }

    pub fn run_files(&self, inputs: &[PathBuf]) -> Result<RunSummary> {
        let mut total_violations = 0usize;
        let mut had_error = false;
        for p in inputs {
            match self.run_file(p) {
                Ok(n) => total_violations += n,
                Err(_) => had_error = true,
            }
        }
        Ok(RunSummary {
            violations: total_violations,
            had_error,
        })
    }

    pub fn run_file(&self, input: &Path) -> Result<usize> {
        let (normalized_text, input_path) = read_input(input)?;
        let driver = SvDriver::new(&self.cfg.svparser);
        let artifacts = driver.parse_text(&normalized_text, &input_path);
        let mut all: Vec<Violation> = Vec::new();

        for stage in &self.cfg.stages.enabled {
            log_event(Ev::new(Event::StageStart, &input_path.to_string_lossy()).with_stage(stage.as_str()));
            let payload = payload_for(stage, &artifacts);
            let vs = run_plugin_once(self.cfg, stage.as_str(), &input_path, payload)?;
            all.extend(vs);
            log_event(Ev::new(Event::StageDone, &input_path.to_string_lossy()).with_stage(stage.as_str()));
        }

        print_violations(&all, &input_path);
        Ok(all.len())
    }
}

fn payload_for(stage: &Stage, a: &ParseArtifacts) -> serde_json::Value {
    match stage {
        Stage::RawText => json!({ "text": a.raw_text }),
        Stage::PpText => {
            json!({ "text": a.pp_text, "defines": a.defines.iter().map(|d| json!({ "name": d.name, "value": d.value })).collect::<Vec<_>>() })
        }
        Stage::Cst => json!({ "has_cst": a.has_cst }),
        Stage::Ast => json!({ "decls": a.ast.decls, "refs": a.ast.refs, "symbols": a.ast.symbols }),
    }
}

trait EvExt<'a> {
    fn with_stage(self, s: &'a str) -> Self;
}

impl<'a> EvExt<'a> for Ev<'a> {
    fn with_stage(mut self, s: &'a str) -> Self {
        self.stage = Some(s);
        self
    }
}
