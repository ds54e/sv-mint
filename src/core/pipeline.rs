use crate::config::{read_input, Config, TransportOnExceed};
use crate::core::payload::{payload_for, StagePayload};
use crate::core::size_guard::{
    enforce_request_size, enforce_response_size, OnExceed, SizePolicy, StageOutcome, StageStatus,
};
use crate::diag::event::{Ev, Event};
use crate::diag::logging::log_event;
use crate::output::print_violations;
use crate::plugin::client::{PythonHost, RuleDispatch};
use crate::svparser::SvDriver;
use crate::types::{Location, Severity, Stage, Violation};
use anyhow::{anyhow, Result};
use serde::Serialize;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Instant;
use tracing::{debug, error};

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
        let (input_text, input_path) = read_input(input)?;
        let driver = SvDriver::new(&self.cfg.svparser);
        let artifacts = match driver.parse_text(&input_text.raw, &input_text.normalized, &input_path) {
            Ok(a) => a,
            Err(e) => {
                let violation = Violation {
                    rule_id: "sys.parse.failed".to_string(),
                    severity: Severity::Error,
                    message: e.to_string(),
                    location: Location {
                        line: 1,
                        col: 1,
                        end_line: 1,
                        end_col: 1,
                        file: Some(input_path.to_string_lossy().into_owned()),
                    },
                };
                print_violations(&[violation], &input_path);
                return Ok(1);
            }
        };
        let mut all: Vec<Violation> = Vec::new();
        let stage_rule_map = build_stage_rule_map(self.cfg);

        for stage in &self.cfg.stages.enabled {
            log_event(Ev::new(Event::StageStart, &input_path.to_string_lossy()).with_stage(stage.as_str()));
            let payload = payload_for(stage, &artifacts);
            let rules_for_stage = stage_rule_map.get(stage).expect("stage rule map missing entry");
            let request_rules = RuleDispatch {
                enabled: &rules_for_stage.enabled,
                disabled: &rules_for_stage.disabled,
            };
            let invocation = StageRequest {
                kind: "run_stage",
                stage: stage.as_str(),
                path: &input_path,
                payload: &payload,
                rules: request_rules,
            };
            let policy = self.size_policy(stage);
            if let Err(outcome) = enforce_request_size(stage.as_str(), &invocation, &policy) {
                all.extend(outcome.violations.iter().cloned());
                record_outcome(&input_path, &outcome);
                log_event(Ev::new(Event::StageDone, &input_path.to_string_lossy()).with_stage(stage.as_str()));
                if matches!(outcome.status, StageStatus::Failed) || outcome.fail_ci {
                    print_violations(&all, &input_path);
                    return Err(anyhow!(format!("stage {} aborted", stage.as_str())));
                }
                continue;
            }
            let t0 = Instant::now();
            let run_rules = RuleDispatch {
                enabled: &rules_for_stage.enabled,
                disabled: &rules_for_stage.disabled,
            };
            let result = host
                .run_stage(stage, &input_path, payload, run_rules)
                .map_err(anyhow::Error::new)?;
            if let Err(mut outcome) = enforce_response_size(stage.as_str(), result.response_bytes, &policy) {
                all.extend(outcome.violations.iter().cloned());
                outcome.duration_ms = t0.elapsed().as_millis() as u64;
                record_outcome(&input_path, &outcome);
                log_event(Ev::new(Event::StageDone, &input_path.to_string_lossy()).with_stage(stage.as_str()));
                if matches!(outcome.status, StageStatus::Failed) || outcome.fail_ci {
                    print_violations(&all, &input_path);
                    return Err(anyhow!(format!("stage {} aborted", stage.as_str())));
                }
                continue;
            }
            let outcome = StageOutcome {
                stage: stage.as_str().to_string(),
                status: StageStatus::Ran,
                violations: result.violations,
                duration_ms: t0.elapsed().as_millis() as u64,
                fail_ci: false,
            };
            all.extend(outcome.violations.iter().cloned());
            record_outcome(&input_path, &outcome);
            log_event(Ev::new(Event::StageDone, &input_path.to_string_lossy()).with_stage(stage.as_str()));
        }

        print_violations(&all, &input_path);
        Ok(all.len())
    }
}

fn is_required_stage(cfg: &Config, stage: &Stage) -> bool {
    if cfg.stages.required.is_empty() {
        matches!(stage, Stage::RawText | Stage::PpText)
    } else {
        cfg.stages.required.contains(stage)
    }
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
    let warn_request_bytes = cfg
        .transport
        .max_request_bytes
        .saturating_sub(cfg.transport.warn_margin_bytes);
    let on_exceed = if required {
        OnExceed::Error
    } else {
        match cfg.transport.on_exceed {
            TransportOnExceed::Skip => OnExceed::Skip,
            TransportOnExceed::Error => OnExceed::Error,
        }
    };
    SizePolicy {
        max_request_bytes: cfg.transport.max_request_bytes,
        warn_request_bytes,
        max_response_bytes: cfg.transport.max_response_bytes,
        on_exceed,
        fail_ci_on_skip: cfg.transport.fail_ci_on_skip,
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
    rules: RuleDispatch<'a>,
}

#[derive(Default)]
struct StageRuleSet {
    enabled: Vec<String>,
    disabled: Vec<String>,
}

fn build_stage_rule_map(cfg: &Config) -> HashMap<Stage, StageRuleSet> {
    let mut map: HashMap<Stage, StageRuleSet> = HashMap::new();
    const ALL_STAGES: [Stage; 4] = [Stage::RawText, Stage::PpText, Stage::Cst, Stage::Ast];
    for stage in ALL_STAGES {
        map.entry(stage).or_insert_with(StageRuleSet::default);
    }
    for rule in &cfg.rule {
        let entry = map.entry(rule.stage).or_insert_with(StageRuleSet::default);
        if rule.enabled {
            entry.enabled.push(rule.id.clone());
        } else {
            entry.disabled.push(rule.id.clone());
        }
    }
    map
}
