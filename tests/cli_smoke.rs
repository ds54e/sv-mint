use assert_cmd::Command;
use predicates::prelude::{PredicateBooleanExt, PredicateBoxExt};
use predicates::str::contains;

fn run_fixture(path: &str, fragment: &str) {
    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("sv-mint"));
    cmd.arg(path);
    cmd.assert().failure().stdout(contains(fragment));
}

fn run_fixture_success(path: &str) {
    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("sv-mint"));
    cmd.arg(path);
    cmd.assert().success();
}

fn run_with_config(path: &str, config: &str, expected: &[&str]) {
    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("sv-mint"));
    cmd.arg("--config").arg(config).arg(path);
    let mut pred = contains(expected[0]).boxed();
    for frag in &expected[1..] {
        pred = pred.and(contains(*frag)).boxed();
    }
    cmd.assert().failure().stdout(pred);
}

#[test]
fn detects_unused_net_violation() {
    run_fixture("fixtures/unused_net_violation.sv", "decl.unused_net");
}

#[test]
fn allows_marked_unused_net() {
    run_fixture_success("fixtures/unused_net_compliant.sv");
}

#[test]
fn detects_unused_param_violation() {
    run_fixture("fixtures/unused_param_violation.sv", "decl.unused_param");
}

#[test]
fn allows_marked_unused_param() {
    run_fixture_success("fixtures/unused_param_compliant.sv");
}

#[test]
fn detects_unused_var_violation() {
    run_fixture("fixtures/unused_var_violation.sv", "decl.unused_var");
}

#[test]
fn allows_marked_unused_var() {
    run_fixture_success("fixtures/unused_var_compliant.sv");
}

#[test]
fn detects_multiple_nonblocking_assignments() {
    run_fixture("fixtures/multiple_nonblocking.sv", "flow.multiple_nonblocking");
}

#[test]
fn allows_unique_case_without_default() {
    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("sv-mint"));
    cmd.arg("--disable").arg("module.named_ports_required");
    cmd.arg("fixtures/case_missing_default_unique_ok.sv");
    cmd.assert().success();
}

#[test]
fn detects_module_inst_violations() {
    run_fixture("fixtures/module_inst_violation.sv", "module.named_ports_required");
}

#[test]
fn detects_typedef_violations() {
    run_fixture("fixtures/typedef_violation.sv", "typedef.enum_suffix");
}

#[test]
fn detects_function_scope_violations() {
    run_fixture("fixtures/function_scope_violation.sv", "style.function_scope");
}

#[test]
fn detects_macro_undef_violations() {
    run_fixture("fixtures/macro_violation.sv", "macro.missing_undef");
}

#[test]
fn detects_unused_macro() {
    run_fixture("fixtures/macro_unused.sv", "macro.unused_macro");
    run_fixture_success("fixtures/macro_used.sv");
}

#[test]
fn reports_include_file_path() {
    run_with_config(
        "fixtures/include_top.sv",
        "tests/include_config.toml",
        &["include_child.sv", "decl.unused_var"],
    );
}
