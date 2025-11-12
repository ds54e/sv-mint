use assert_cmd::Command;
use predicates::prelude::{PredicateBooleanExt, PredicateBoxExt};
use predicates::str::contains;
use std::io::Write;
use tempfile::NamedTempFile;

fn run_fixture(path: &str, fragment: &str) {
    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("sv-mint"));
    cmd.arg(path);
    cmd.assert().failure().stdout(contains(fragment));
}

fn run_temp_source(content: &str, expected: &[&str]) {
    let mut temp = NamedTempFile::new().expect("tempfile");
    temp.write_all(content.as_bytes()).expect("write");
    let path = temp.into_temp_path();
    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("sv-mint"));
    cmd.arg(path.as_os_str());
    let mut pred = contains(expected[0]).boxed();
    for frag in &expected[1..] {
        pred = pred.and(contains(*frag)).boxed();
    }
    cmd.assert().failure().stdout(pred);
    path.close().expect("cleanup");
}

#[test]
fn detects_line_length_violation() {
    run_fixture("fixtures/format_line_length_violation.sv", "format.line_length");
}

#[test]
fn detects_port_wildcard() {
    run_fixture("fixtures/port_wildcard_violation.sv", "module.no_port_wildcard");
}

#[test]
fn detects_if_without_begin() {
    run_fixture("fixtures/if_without_begin.sv", "format.begin_required");
}

#[test]
fn detects_whitespace_violations() {
    run_fixture("fixtures/whitespace_violations.sv", "format.no_tabs");
}

#[test]
fn detects_spacing_violations() {
    run_fixture("fixtures/spacing_violations.sv", "format.call_spacing");
}

#[test]
fn detects_ascii_and_newline_violations() {
    run_temp_source(
        "module ascii_check; // é",
        &["format.ascii_only", "format.final_newline"],
    );
}

#[test]
fn detects_macro_spacing() {
    run_temp_source(
        "`define FOO(x) x\nmodule macro_spacing;\ninitial begin\n  `FOO (x)\nend\nendmodule\n",
        &["format.macro_spacing"],
    );
}
