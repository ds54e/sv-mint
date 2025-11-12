use assert_cmd::Command;
use predicates::str::contains;

fn run_fixture(path: &str, fragment: &str) {
    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("sv-mint"));
    cmd.arg(path);
    cmd.assert()
        .failure()
        .stdout(contains(fragment));
}

#[test]
fn detects_line_length_violation() {
    run_fixture(
        "fixtures/format_line_length_violation.sv",
        "format.line_length",
    );
}

#[test]
fn detects_port_wildcard() {
    run_fixture(
        "fixtures/port_wildcard_violation.sv",
        "module.no_port_wildcard",
    );
}

#[test]
fn detects_if_without_begin() {
    run_fixture("fixtures/if_without_begin.sv", "format.begin_required");
}
