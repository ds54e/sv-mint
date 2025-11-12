use crate::config::{read_input, Config};
use crate::core::payload::{payload_for, StagePayload};
use crate::core::size_guard::{enforce_request_size, OnExceed, SizePolicy, StageOutcome, StageStatus};
use crate::diag::event::{Ev, Event};
use crate::diag::logging::log_event;
use crate::output::print_violations;
use crate::plugin::client::PythonHost;
use crate::svparser::SvDriver;
use crate::types::{Stage, Violation};
use anyhow::Result;
use serde::Serialize;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Instant;
use tracing::{debug, error};

const MAX_REQ_BYTES: usize = 16_000_000;
const WARN_REQ_BYTES: usize = 12_000_000;
const MAX_RESP_BYTES: usize = 16_000_000;

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
        let total = inputs.len();
        let index = AtomicUsize::new(0);
        let mut results: Vec<Result<RunSummary>> = Vec::new();
        std::thread::scope(|scope| {
            let mut handles = Vec::new();
            for _ in 0..worker_count {
                let pipeline = Pipeline { cfg: self.cfg };
                let counter = &index;
                handles.push(scope.spawn(move || pipeline.run_worker(inputs, total, counter)));
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

    fn run_worker(&self, inputs: &[PathBuf], total: usize, counter: &AtomicUsize) -> Result<RunSummary> {
        let mut host = PythonHost::start(self.cfg).map_err(anyhow::Error::new)?;
        let mut summary = RunSummary {
            violations: 0,
            had_error: false,
        };
        loop {
            let next = counter.fetch_add(1, Ordering::SeqCst);
            if next >= total {
                break;
            }
            let path = &inputs[next];
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
            let invocation = StageRequest {
                kind: "run_stage",
                stage: stage.as_str(),
                path: &input_path,
                payload: &payload,
            };
            let policy = self.size_policy(stage);
            if let Err(outcome) = enforce_request_size(stage.as_str(), &invocation, &policy) {
                all.extend(outcome.violations.iter().cloned());
                record_outcome(&input_path, &outcome);
                log_event(Ev::new(Event::StageDone, &input_path.to_string_lossy()).with_stage(stage.as_str()));
                if matches!(outcome.status, StageStatus::Failed) {
                    print_violations(&all, &input_path);
                    return Ok(all.len());
                }
                continue;
            }
            let t0 = Instant::now();
            let vs = host
                .run_stage(stage, &input_path, payload)
                .map_err(anyhow::Error::new)?;
            let outcome = StageOutcome {
                stage: stage.as_str().to_string(),
                status: StageStatus::Ran,
                violations: vs,
                duration_ms: t0.elapsed().as_millis() as u64,
            };
            all.extend(outcome.violations.iter().cloned());
            record_outcome(&input_path, &outcome);
            log_event(Ev::new(Event::StageDone, &input_path.to_string_lossy()).with_stage(stage.as_str()));
        }

        print_violations(&all, &input_path);
        Ok(all.len())
    }
}

fn is_required_stage(_cfg: &Config, stage: &Stage) -> bool {
    matches!(stage, Stage::RawText | Stage::PpText)
}

fn record_outcome(path: &Path, outcome: &StageOutcome) {
    let status = match outcome.status {
        StageStatus::Ran => "ran",
        StageStatus::Skipped => "skipped",
        StageStatus::Failed => "failed",
    };
    debug!(
        target: "sv-mint::stage",
        path = %path.display(),
        stage = outcome.stage.as_str(),
        status,
        violations = outcome.violations.len(),
        duration_ms = outcome.duration_ms,
    );
}

fn size_policy_for_stage(cfg: &Config, stage: &Stage) -> SizePolicy {
    let required = is_required_stage(cfg, stage);
    SizePolicy {
        max_request_bytes: MAX_REQ_BYTES,
        warn_request_bytes: WARN_REQ_BYTES,
        max_response_bytes: MAX_RESP_BYTES,
        on_exceed: if required { OnExceed::Error } else { OnExceed::Skip },
        fail_ci_on_skip: false,
        is_required_stage: required,
    }
}

impl<'a> Pipeline<'a> {
    fn size_policy(&self, stage: &Stage) -> SizePolicy {
        size_policy_for_stage(self.cfg, stage)
    }
}

#[derive(Serialize)]
struct StageRequest<'a> {
    kind: &'static str,
    stage: &'a str,
    path: &'a Path,
    #[serde(borrow)]
    payload: &'a StagePayload<'a>,
}
