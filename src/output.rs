use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::io::Write;

#[derive(Serialize, Deserialize)]
pub struct Location {
    pub line: u32,
    pub col: u32,
    pub end_line: u32,
    pub end_col: u32,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Severity {
    Error,
    Warning,
    Info,
}

#[derive(Serialize, Deserialize)]
pub struct Violation {
    pub rule_id: String,
    pub severity: Severity,
    pub message: String,
    pub location: Location,
}

pub fn print_violations(path: &str, text: &str, vs: &[Violation]) -> Result<()> {
    let lines: Vec<&str> = text.split('\n').collect();
    let mut out = std::io::BufWriter::new(std::io::stdout().lock());
    for v in vs {
        let idx = v.location.line.saturating_sub(1) as usize;
        let excerpt = lines.get(idx).copied().unwrap_or_default();
        writeln!(
            &mut out,
            "{}:{}:{}: [{}] {}: {}",
            path,
            v.location.line,
            v.location.col,
            sev_str(&v.severity),
            v.rule_id,
            v.message
        )?;
        writeln!(&mut out, "    > {}", excerpt)?;
    }
    out.flush()?;
    Ok(())
}

fn sev_str(s: &Severity) -> &'static str {
    match s {
        Severity::Error => "error",
        Severity::Warning => "warning",
        Severity::Info => "info",
    }
}
