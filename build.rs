use std::collections::HashSet;
use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};

fn main() {
    println!("cargo:rerun-if-changed=fixtures/rules");
    let out_dir = std::env::var("OUT_DIR").expect("OUT_DIR");
    let dest = PathBuf::from(out_dir).join("cli_fixtures.rs");
    let cases = collect_cases(Path::new("fixtures/rules"));
    let mut file = File::create(dest).expect("create cli_fixtures.rs");
    for case in cases {
        if case.expect_fail {
            writeln!(
                file,
                "#[test]\nfn {}() {{\n    expect_fail(\"{}\", \"{}\");\n}}\n",
                case.fn_name, case.path, case.rule
            )
            .unwrap();
        } else {
            writeln!(
                file,
                "#[test]\nfn {}() {{\n    expect_pass(\"{}\");\n}}\n",
                case.fn_name, case.path
            )
            .unwrap();
        }
    }
}

struct Case {
    rule: String,
    stem: String,
    path: String,
    expect_fail: bool,
    fn_name: String,
}

fn collect_cases(root: &Path) -> Vec<Case> {
    let mut out = Vec::new();
    let mut rules: Vec<_> = fs::read_dir(root)
        .expect("read fixtures/rules")
        .filter_map(|e| e.ok())
        .filter(|e| e.path().is_dir())
        .collect();
    rules.sort_by_key(|e| e.file_name());
    for entry in rules {
        let rule = entry.file_name().to_string_lossy().to_string();
        let mut files: Vec<_> = fs::read_dir(entry.path())
            .unwrap()
            .filter_map(|e| e.ok())
            .filter(|e| e.path().is_file())
            .collect();
        files.sort_by_key(|e| e.file_name());
        for file in files {
            let name = file.file_name().to_string_lossy().to_string();
            if !name.ends_with(".sv") {
                continue;
            }
            let stem = file
                .path()
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or(&name)
                .to_string();
            let expect_fail = stem.to_lowercase().contains("bad");
            let path = file.path();
            let path_str = path.to_string_lossy().replace('\\', "/");
            out.push(Case {
                rule: rule.clone(),
                stem,
                path: path_str,
                expect_fail,
                fn_name: String::new(),
            });
        }
    }
    assign_fn_names(&mut out);
    out
}

fn assign_fn_names(cases: &mut [Case]) {
    let mut seen = HashSet::new();
    for case in cases.iter_mut() {
        let base = format!("rule_{}_{}", sanitize(&case.rule), sanitize(&case.stem));
        let mut name = base.clone();
        let mut idx = 2;
        while !seen.insert(name.clone()) {
            name = format!("{}_{}", base, idx);
            idx += 1;
        }
        case.fn_name = name;
    }
}

fn sanitize(raw: &str) -> String {
    let mut out = String::new();
    for ch in raw.chars() {
        if ch.is_ascii_alphanumeric() {
            out.push(ch.to_ascii_lowercase());
        } else {
            out.push('_');
        }
    }
    if out.is_empty() {
        out.push('_');
    }
    if out.starts_with(|c: char| c.is_ascii_digit()) {
        out.insert(0, '_');
    }
    out
}
