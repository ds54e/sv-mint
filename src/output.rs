use crate::types::{Severity, Violation};
use std::fs;
use std::path::Path;
#[allow(dead_code)]
pub fn normalize_windows_path_for_output(p: &str) -> String {
    if let Some(rest) = p.strip_prefix(r"\\?\UNC\") {
        return format!(r"\\{}", rest);
    }
    if let Some(rest) = p.strip_prefix(r"\\?\") {
        return rest.to_string();
    }
    p.to_string()
}

fn truncate_preview_utf8(s: &str, max: usize) -> String {
    if s.len() <= max {
        return s.to_owned();
    }
    let mut end = max.min(s.len());
    while end > 0 && !s.is_char_boundary(end) {
        end -= 1;
    }
    if end == 0 {
        return String::new();
    }
    let mut t = s[..end].to_owned();
    t.push_str(" ...");
    t
}

fn read_file_text_lf(path: &Path) -> Option<String> {
    let bytes = fs::read(path).ok()?;
    let s = String::from_utf8(bytes).ok()?;
    let s = s.replace("\r\n", "\n").replace('\r', "\n");
    Some(s)
}

fn line_starts(text: &str) -> Vec<usize> {
    let mut v = Vec::with_capacity(1024);
    v.push(0);
    for (i, b) in text.as_bytes().iter().enumerate() {
        if *b == b'\n' && i + 1 < text.len() {
            v.push(i + 1);
        }
    }
    v
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

pub fn make_line_excerpt_from_file(path: &Path, line: usize, tabstop: usize, max_cols: usize) -> String {
    let text = match read_file_text_lf(path) {
        Some(t) => t,
        None => return String::new(),
    };
    let starts = line_starts(&text);
    let s = get_line_str(&text, &starts, line);
    let mut cols = 0usize;
    let mut out = String::new();
    for ch in s.chars() {
        let w = if ch == '\t' { tabstop } else { 1 };
        if cols + w > max_cols {
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
        let excerpt = make_line_excerpt_from_file(path, line as usize, 8, 200);
        if !excerpt.is_empty() {
            println!("    > {}", truncate_preview_utf8(&excerpt, 200));
        }
    }
}
