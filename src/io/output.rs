use crate::errors::OutputError;
use crate::types::{Severity, Violation};
use std::fs;
use std::path::Path;

pub fn read_file_to_string(path: &Path) -> Result<String, OutputError> {
    let bytes = fs::read(path).map_err(|e| OutputError::ReadFailed {
        path: path.display().to_string(),
        source: Some(e),
    })?;
    let s = String::from_utf8(bytes).map_err(|_| OutputError::InvalidUtf8 {
        path: path.display().to_string(),
        source: None,
    })?;
    Ok(s)
}

pub fn print_violations(violations: &[Violation], input_path: &Path) {
    for v in violations {
        let sev = match v.severity {
            Severity::Error => "error",
            Severity::Warning => "warning",
            Severity::Info => "info",
        };
        let line = v.location.line.max(1);
        let col = v.location.col.max(1);
        println!(
            "{}:{}:{}: [{}] {}: {}",
            input_path.display(),
            line,
            col,
            sev,
            v.rule_id,
            v.message
        );
    }
}
