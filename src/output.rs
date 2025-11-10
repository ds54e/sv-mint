use crate::errors::OutputError;
use crate::textutil::truncate_preview_utf8;
use crate::textutil::{line_starts, normalize_lf, strip_bom};
use crate::types::{Severity, Violation};
use std::fs;
use std::path::Path;

const TABSTOP: usize = 8;
const MAX_COLS: usize = 200;

pub fn normalize_windows_path_for_output(p: &str) -> String {
    if let Some(rest) = p.strip_prefix(r"\\?\UNC\") {
        return format!(r"\\{}", rest);
    }
    if let Some(rest) = p.strip_prefix(r"\\?\") {
        return rest.to_string();
    }
    p.to_string()
}

fn read_file_text_lf(path: &Path) -> Result<String, OutputError> {
    let bytes = fs::read(path).map_err(|e| OutputError::ReadFailed {
        path: path.display().to_string(),
        source: Some(e),
    })?;
    let s = String::from_utf8(bytes).map_err(|_| OutputError::InvalidUtf8 {
        path: path.display().to_string(),
    })?;
    Ok(normalize_lf(strip_bom(s)))
}

fn get_line_str<'a>(text: &'a str, starts: &[usize], line: usize) -> &'a str {
    if line == 0 {
        return "";
    }
    let i = line - 1;
    if i >= starts.len() {
        return "";
    }
    let start = starts[i];
    let end = if i + 1 < starts.len() {
        starts[i + 1].saturating_sub(1)
    } else {
        text.len()
    };
    if start <= end && end <= text.len() {
        &text[start..end]
    } else {
        ""
    }
}

pub fn make_line_excerpt_from_file(path: &Path, line: usize) -> String {
    let text = match read_file_text_lf(path) {
        Ok(t) => t,
        Err(_) => return String::new(),
    };
    let starts = line_starts(&text);
    let s = get_line_str(&text, &starts, line);
    let mut cols = 0usize;
    let mut out = String::new();
    for ch in s.chars() {
        let w = if ch == '\t' { TABSTOP } else { 1 };
        if cols + w > MAX_COLS {
            out.push_str(" ...");
            return out;
        }
        out.push(ch);
        cols += w;
    }
    out
}

pub fn print_violations(path: &Path, violations: &[Violation]) {
    let display_path = normalize_windows_path_for_output(&path.to_string_lossy());
    for v in violations {
        let line = v.location.line.max(1);
        let col = v.location.col.max(1);
        println!(
            "{}:{}:{}: [{}] {}: {}",
            display_path,
            line,
            col,
            match &v.severity {
                Severity::Error => "error",
                Severity::Warning => "warning",
                Severity::Info => "info",
            },
            v.rule_id,
            v.message
        );
        let excerpt = make_line_excerpt_from_file(path, line as usize);
        if !excerpt.is_empty() {
            println!("    > {}", truncate_preview_utf8(&excerpt, MAX_COLS));
        }
    }
}
