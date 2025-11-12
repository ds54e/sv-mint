use crate::config::{read_input, Config};
use crate::diag::event::{Ev, Event};
use crate::diag::logging::log_event;
use crate::output::print_violations;
use crate::plugin::client::PythonHost;
use crate::sv::model::ParseArtifacts;
use crate::svparser::SvDriver;
use crate::types::{Location, Severity, Stage, Violation};
use anyhow::Result;
use serde_json::json;
use std::path::{Path, PathBuf};

const MAX_REQ_BYTES: usize = 16_000_000;
const WARN_REQ_BYTES: usize = 12_000_000;

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
        let mut host = PythonHost::start(self.cfg).map_err(anyhow::Error::new)?;
        for p in inputs {
            match self.run_file_with_host(p, &mut host) {
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
        let mut host = PythonHost::start(self.cfg).map_err(anyhow::Error::new)?;
        self.run_file_with_host(input, &mut host)
    }

    fn run_file_with_host(&self, input: &Path, host: &mut PythonHost) -> Result<usize> {
        let (normalized_text, input_path) = read_input(input)?;
        let driver = SvDriver::new(&self.cfg.svparser);
        let artifacts = driver.parse_text(&normalized_text, &input_path);
        let mut all: Vec<Violation> = Vec::new();

        for stage in &self.cfg.stages.enabled {
            log_event(Ev::new(Event::StageStart, &input_path.to_string_lossy()).with_stage(stage.as_str()));
            let payload = payload_for(stage, &artifacts);
            let req = json!({ "stage": stage.as_str(), "path": input_path, "payload": payload });
            let req_bytes = serde_json::to_vec(&req).unwrap_or_default().len();
            if (WARN_REQ_BYTES..=MAX_REQ_BYTES).contains(&req_bytes) {
                log::warn!(
                    "{} payload nearing limit: {} / {}",
                    stage.as_str(),
                    req_bytes,
                    MAX_REQ_BYTES
                );
            }
            if req_bytes > MAX_REQ_BYTES {
                let sev = if is_required_stage(self.cfg, stage) {
                    Severity::Error
                } else {
                    Severity::Warning
                };
                let is_err = matches!(sev, Severity::Error);
                let v = Violation {
                    rule_id: "sys.stage.skipped.size".to_string(),
                    severity: sev,
                    message: format!(
                        "Stage '{}' skipped: request payload {} bytes exceeds limit {} bytes.",
                        stage.as_str(),
                        req_bytes,
                        MAX_REQ_BYTES
                    ),
                    location: Location {
                        line: 1,
                        col: 1,
                        end_line: 1,
                        end_col: 1,
                    },
                };
                all.push(v);
                log_event(Ev::new(Event::StageDone, &input_path.to_string_lossy()).with_stage(stage.as_str()));
                if is_err {
                    print_violations(&all, &input_path);
                    return Ok(all.len());
                }
                continue;
            }
            let vs = host
                .run_stage(stage, &input_path, payload)
                .map_err(anyhow::Error::new)?;
            all.extend(vs);
            log_event(Ev::new(Event::StageDone, &input_path.to_string_lossy()).with_stage(stage.as_str()));
        }

        print_violations(&all, &input_path);
        Ok(all.len())
    }
}

fn is_required_stage(_cfg: &Config, stage: &Stage) -> bool {
    matches!(stage, Stage::RawText | Stage::PpText)
}

fn payload_for(stage: &Stage, a: &ParseArtifacts) -> serde_json::Value {
    match stage {
        Stage::RawText => json!({ "text": a.raw_text }),
        Stage::PpText => {
            json!({ "text": a.pp_text, "defines": a.defines.iter().map(|d| json!({ "name": d.name, "value": d.value })).collect::<Vec<_>>() })
        }
        Stage::Cst => {
            if let Some(ir) = &a.cst_ir {
                serde_json::json!({ "mode": "inline", "cst_ir": ir })
            } else {
                serde_json::json!({ "mode": "none", "has_cst": a.has_cst })
            }
        }
        Stage::Ast => {
            json!({ "schema_version": 1, "decls": a.ast.decls, "refs": a.ast.refs, "symbols": a.ast.symbols, "assigns": a.ast.assigns, "scopes": [], "pp_text": a.ast.pp_text })
        }
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
