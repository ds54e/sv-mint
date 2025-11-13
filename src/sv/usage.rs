use regex::Regex;
use crate::sv::model::{AstSummary, Symbol, Reference, Scope, SymbolKind, PortDir, ScopeKind, Location};

fn byte_to_line_col(text: &str, byte_idx: usize) -> (u32, u32) {
    let mut line = 1usize;
    let mut col = 1usize;
    let mut count = 0usize;
    for ch in text.chars() {
        let len = ch.len_utf8();
        if count >= byte_idx { break; }
        if ch == '\n' {
            line += 1;
            col = 1;
        } else {
            col += 1;
        }
        count += len;
    }
    (line as u32, col as u32)
}

fn make_loc(file: &str, text: &str, start: usize, end: usize) -> Location {
    let (line, col) = byte_to_line_col(text, start);
    let (end_line, end_col) = byte_to_line_col(text, end);
    Location {
        line,
        col,
        end_line,
        end_col,
        file: Some(file.to_string()),
    }
}

fn split_names(s: &str) -> Vec<String> {
    s.split(',')
        .map(|v| v.trim())
        .filter(|v| !v.is_empty())
        .map(|v| {
            let v = v.split('=').next().unwrap_or(v).trim();
            let v = v.rsplit(|c: char| c.is_whitespace() || c == '[').next().unwrap_or(v).trim();
            v.to_string()
        })
        .filter(|v| Regex::new(r"^[A-Za-z_][A-Za-z0-9_]*$").unwrap().is_match(v))
        .collect()
}

pub fn collect_from_text(file: &str, text: &str) -> (Vec<Symbol>, Vec<Reference>, Vec<Scope>) {
    let mut symbols: Vec<Symbol> = Vec::new();
    let refs: Vec<Reference> = Vec::new();
    let mut scopes: Vec<Scope> = Vec::new();
    let mut next_sid: u32 = 1;
    let mut next_scope_id: u32 = 1;

    let module_re = Regex::new(r"(?s)\bmodule\s+([A-Za-z_][A-Za-z0-9_]*)\s*(?:#\s*\([^;]*?\))?\s*\((.*?)\)\s*;").unwrap();
    for cap in module_re.captures_iter(text) {
        let mname = cap.get(1).unwrap().as_str().to_string();
        let mspan = cap.get(0).unwrap();
        let header = cap.get(2).unwrap().as_str();
        let scope_id = next_scope_id;
        next_scope_id += 1;
        let loc = make_loc(file, text, mspan.start(), mspan.end());
        scopes.push(Scope { id: scope_id, kind: ScopeKind::Module, name: mname.clone(), parent: 0 });

        let port_re = Regex::new(r"\b(input|output|inout)\b[^,;\)\(]*?\b([A-Za-z_][A-Za-z0-9_]*)\b").unwrap();
        for p in port_re.captures_iter(header) {
            let dir_s = p.get(1).unwrap().as_str();
            let name = p.get(2).unwrap().as_str().to_string();
            let dir = match dir_s { "input" => Some(PortDir::Input), "output" => Some(PortDir::Output), _ => Some(PortDir::Inout) };
            let sloc = loc.clone();
            symbols.push(Symbol { id: next_sid, name, kind: SymbolKind::Port, dir, scope_id, decl_loc: sloc, type_ref: None });
            next_sid += 1;
        }
    }

    let param_re = Regex::new(r"(?m)^\s*(parameter|localparam)\b([^;]*);\s*$").unwrap();
    for cap in param_re.captures_iter(text) {
        let full = cap.get(0).unwrap();
        let kind_s = cap.get(1).unwrap().as_str();
        let body = cap.get(2).unwrap().as_str();
        let names = split_names(body);
        for n in names {
            let loc = make_loc(file, text, full.start(), full.end());
            let kind = if kind_s == "parameter" { SymbolKind::Param } else { SymbolKind::Localparam };
            symbols.push(Symbol { id: next_sid, name: n, kind, dir: None, scope_id: 0, decl_loc: loc, type_ref: None });
            next_sid += 1;
        }
    }

    let typedef_re = Regex::new(r"(?m)^\s*typedef\b[^;]*\b([A-Za-z_][A-Za-z0-9_]*)\s*;\s*$").unwrap();
    for cap in typedef_re.captures_iter(text) {
        let full = cap.get(0).unwrap();
        let name = cap.get(1).unwrap().as_str().to_string();
        let loc = make_loc(file, text, full.start(), full.end());
        symbols.push(Symbol { id: next_sid, name, kind: SymbolKind::Typedef, dir: None, scope_id: 0, decl_loc: loc, type_ref: None });
        next_sid += 1;
    }

    let var_re = Regex::new(r"(?m)^\s*(logic|wire|reg|bit|byte|shortint|int|longint)\b([^;]*);\s*$").unwrap();
    for cap in var_re.captures_iter(text) {
        let full = cap.get(0).unwrap();
        let body = cap.get(2).unwrap().as_str();
        let names = split_names(body);
        for n in names {
            let loc = make_loc(file, text, full.start(), full.end());
            let kind = SymbolKind::Var;
            symbols.push(Symbol { id: next_sid, name: n, kind, dir: None, scope_id: 0, decl_loc: loc, type_ref: None });
            next_sid += 1;
        }
    }

    (symbols, refs, scopes)
}
