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
use tracing::{error, warn};

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
        if inputs.is_empty() {
            return Ok(RunSummary {
                violations: 0,
                had_error: false,
            });
        }
        if inputs.len() == 1 {
            return self.run_file_batch(inputs);
        }
        self.run_files_parallel(inputs)
    }

    pub fn run_file(&self, input: &Path) -> Result<usize> {
        let mut host = PythonHost::start(self.cfg).map_err(anyhow::Error::new)?;
        self.run_file_with_host(input, &mut host)
    }

    fn run_files_parallel(&self, inputs: &[PathBuf]) -> Result<RunSummary> {
        let worker_count = std::thread::available_parallelism()
            .map(|n| n.get())
            .unwrap_or(1)
            .min(inputs.len());
        let chunk_size = (inputs.len() + worker_count - 1) / worker_count;
        let mut results: Vec<Result<RunSummary>> = Vec::new();
        std::thread::scope(|scope| {
            let mut handles = Vec::new();
            for chunk in inputs.chunks(chunk_size) {
                let chunk_paths: Vec<PathBuf> = chunk.iter().cloned().collect();
                let pipeline = Pipeline { cfg: self.cfg };
                handles.push(scope.spawn(move || pipeline.run_file_batch(&chunk_paths)));
            }
            for handle in handles {
                results.push(handle.join().unwrap());
            }
        });
        let mut summary = RunSummary {
            violations: 0,
            had_error: false,
        };
        for res in results {
            match res {
                Ok(r) => {
                    summary.violations += r.violations;
                    summary.had_error |= r.had_error;
                }
                Err(e) => return Err(e),
            }
        }
        Ok(summary)
    }

    fn run_file_batch(&self, inputs: &[PathBuf]) -> Result<RunSummary> {
        let mut host = PythonHost::start(self.cfg).map_err(anyhow::Error::new)?;
        let mut summary = RunSummary {
            violations: 0,
            had_error: false,
        };
        for path in inputs {
            match self.run_file_with_host(path, &mut host) {
                Ok(n) => summary.violations += n,
                Err(e) => {
                    summary.had_error = true;
                    error!("{}: {}", path.display(), e);
                }
            }
        }
        Ok(summary)
    }

    fn run_file_with_host(&self, input: &Path, host: &mut PythonHost) -> Result<usize> {
        let (normalized_text, input_path) = read_input(input)?;
        let driver = SvDriver::new(&self.cfg.svparser);
        let artifacts = driver.parse_text(&normalized_text, &input_path);
        let mut all: Vec<Violation> = Vec::new();

        for stage in &self.cfg.stages.enabled {
            log_event(Ev::new(Event::StageStart, &input_path.to_string_lossy()).with_stage(stage.as_str()));
            let payload = payload_for(stage, &artifacts);
            let req_bytes = serde_json::to_vec(&json!({
                "stage": stage.as_str(),
                "path": input_path,
                "payload": payload.clone(),
            }))
            .map_err(anyhow::Error::new)?
            .len();
            if (WARN_REQ_BYTES..=MAX_REQ_BYTES).contains(&req_bytes) {
                warn!(
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
            json!({
                "schema_version": a.ast.schema_version,
                "decls": a.ast.decls,
                "refs": a.ast.refs,
                "symbols": a.ast.symbols,
                "assigns": a.ast.assigns,
                "scopes": a.ast.scopes,
                "pp_text": a.ast.pp_text
            })
        }
    }
}
