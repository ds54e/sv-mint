use crate::types::{Location, Severity, Violation};
use serde::Serialize;
use tracing::warn;

#[derive(Clone, Debug)]
pub enum OnExceed {
    Skip,
    Error,
}

#[derive(Clone, Debug)]
pub struct SizePolicy {
    pub max_request_bytes: usize,
    pub warn_request_bytes: usize,
    pub max_response_bytes: usize,
    pub on_exceed: OnExceed,
    pub fail_ci_on_skip: bool,
    pub is_required_stage: bool,
}

#[derive(Serialize)]
#[serde(rename_all = "lowercase")]
pub enum StageStatus {
    Ran,
    Skipped,
    Failed,
}

#[derive(Serialize)]
pub struct StageOutcome {
    pub stage: String,
    pub status: StageStatus,
    pub violations: Vec<Violation>,
    pub duration_ms: u64,
    pub fail_ci: bool,
}

pub fn synth_violation_stage_skipped_size(stage: &str, severity: Severity, actual: usize, limit: usize) -> Violation {
    Violation {
        rule_id: "sys.stage.skipped.size".to_string(),
        severity,
        message: format!(
            "Stage '{}' skipped: request payload {} bytes exceeds limit {} bytes.",
            stage, actual, limit
        ),
        location: Location {
            line: 1,
            col: 1,
            end_line: 1,
            end_col: 1,
            file: None,
        },
    }
}

pub fn enforce_request_size<T: Serialize>(stage: &str, req: &T, pol: &SizePolicy) -> Result<Vec<u8>, StageOutcome> {
    let bytes = match serde_json::to_vec(req) {
        Ok(b) => b,
        Err(_) => {
            return Err(StageOutcome {
                stage: stage.to_string(),
                status: StageStatus::Failed,
                violations: vec![Violation {
                    rule_id: "sys.stage.serialize.error".to_string(),
                    severity: Severity::Error,
                    message: format!("Failed to serialize JSON request for stage '{}'", stage),
                    location: Location {
                        line: 1,
                        col: 1,
                        end_line: 1,
                        end_col: 1,
                        file: None,
                    },
                }],
                duration_ms: 0,
                fail_ci: true,
            })
        }
    };
    let len = bytes.len();
    if len >= pol.warn_request_bytes && len <= pol.max_request_bytes {
        warn!("{} payload nearing limit: {} / {}", stage, len, pol.max_request_bytes);
    }
    if len > pol.max_request_bytes {
        let sev = if pol.is_required_stage || matches!(pol.on_exceed, OnExceed::Error) {
            Severity::Error
        } else {
            Severity::Warning
        };
        let is_err = matches!(sev, Severity::Error);
        let v = synth_violation_stage_skipped_size(stage, sev, len, pol.max_request_bytes);
        let status = if is_err {
            StageStatus::Failed
        } else {
            StageStatus::Skipped
        };
        return Err(StageOutcome {
            stage: stage.to_string(),
            status,
            violations: vec![v],
            duration_ms: 0,
            fail_ci: is_err || pol.fail_ci_on_skip,
        });
    }
    Ok(bytes)
}

pub fn enforce_response_size(stage: &str, response_bytes: usize, pol: &SizePolicy) -> Result<(), StageOutcome> {
    if response_bytes > pol.max_response_bytes {
        let v = Violation {
            rule_id: "sys.stage.output.too_large".to_string(),
            severity: Severity::Error,
            message: format!(
                "Stage '{}' output {} bytes exceeds limit {} bytes.",
                stage, response_bytes, pol.max_response_bytes
            ),
            location: Location {
                line: 1,
                col: 1,
                end_line: 1,
                end_col: 1,
                file: None,
            },
        };
        return Err(StageOutcome {
            stage: stage.to_string(),
            status: StageStatus::Failed,
            violations: vec![v],
            duration_ms: 0,
            fail_ci: true,
        });
    }
    Ok(())
}
