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
    run_fixture("fixtures/unused_net_violation.sv", "decl.no_unused_net");
}

#[test]
fn allows_marked_unused_net() {
    run_fixture_success("fixtures/unused_net_compliant.sv");
}

#[test]
fn detects_unused_param_violation() {
    run_fixture("fixtures/unused_param_violation.sv", "decl.no_unused_param");
}

#[test]
fn allows_marked_unused_param() {
    run_fixture_success("fixtures/unused_param_compliant.sv");
}

#[test]
fn detects_unused_var_violation() {
    run_fixture("fixtures/unused_var_violation.sv", "decl.no_unused_var");
}

#[test]
fn allows_marked_unused_var() {
    run_fixture_success("fixtures/unused_var_compliant.sv");
}

#[test]
fn detects_multiple_nonblocking_assignments() {
    run_fixture("fixtures/multiple_nonblocking.sv", "module.require_named_ports");
}

#[test]
fn detects_bare_always() {
    run_fixture("fixtures/always_plain.sv", "lang.always.require_structured");
    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("sv-mint"));
    cmd.arg("--disable").arg("module.require_named_ports");
    cmd.arg("fixtures/always_structured_ok.sv");
    cmd.assert().success();
}

#[test]
fn detects_net_naming_violations() {
    run_fixture("fixtures/net_lower_snake_violation.sv", "naming.net_lower_snake");
    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("sv-mint"));
    cmd.arg("--disable").arg("decl.no_unused_net");
    cmd.arg("fixtures/net_lower_snake_ok.sv");
    cmd.assert().success();
}

#[test]
fn detects_var_naming_violations() {
    run_fixture("fixtures/var_lower_snake_violation.sv", "naming.var_lower_snake");
    run_fixture_success("fixtures/var_lower_snake_ok.sv");
}

#[test]
fn detects_parameter_naming_violations() {
    run_fixture("fixtures/parameter_case_violation.sv", "naming.parameter_uppercase");
    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("sv-mint"));
    cmd.arg("--disable").arg("decl.no_unused_param");
    cmd.arg("fixtures/parameter_case_ok.sv");
    cmd.assert().success();
}

#[test]
fn detects_localparam_naming_violations() {
    run_fixture("fixtures/localparam_case_violation.sv", "naming.localparam_lower_snake");
    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("sv-mint"));
    cmd.arg("--disable").arg("decl.no_unused_param");
    cmd.arg("fixtures/localparam_case_ok.sv");
    cmd.assert().success();
}

#[test]
fn detects_multiple_modules() {
    run_fixture(
        "fixtures/multiple_modules_violation.sv",
        "module.single_module_per_file",
    );
    run_fixture_success("fixtures/multiple_modules_ok.sv");
}

#[test]
fn detects_filename_mismatch() {
    run_fixture("fixtures/module_filename_mismatch.sv", "module.name_matches_file");
    run_fixture_success("fixtures/module_filename_match_ok.sv");
}

#[test]
fn allows_unique_case_without_default() {
    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("sv-mint"));
    cmd.arg("--disable").arg("module.require_named_ports");
    cmd.arg("fixtures/case_missing_default_unique_ok.sv");
    cmd.assert().success();
}

#[test]
fn detects_module_inst_violations() {
    run_fixture("fixtures/module_inst_violation.sv", "module.require_named_ports");
}

#[test]
fn detects_typedef_violations() {
    run_fixture("fixtures/typedef_violation.sv", "typedef.enum_name_lower_snake_e");
}

#[test]
fn detects_function_scope_violations() {
    run_fixture("fixtures/function_scope_violation.sv", "style.require_function_scope");
}

#[test]
fn detects_macro_undef_violations() {
    run_fixture("fixtures/macro_violation.sv", "macro.require_trailing_undef");
}

#[test]
fn detects_define_upper_violations() {
    run_fixture("fixtures/macro_define_upper_violation.sv", "macro.define_uppercase");
    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("sv-mint"));
    cmd.arg("--disable").arg("macro.no_unused_macro");
    cmd.arg("--disable").arg("macro.require_trailing_undef");
    cmd.arg("fixtures/macro_define_upper_ok.sv");
    cmd.assert().success();
}

#[test]
fn detects_unused_macro() {
    run_fixture("fixtures/macro_unused.sv", "macro.no_unused_macro");
    run_fixture_success("fixtures/macro_used.sv");
}

#[test]
fn reports_include_file_path() {
    run_with_config(
        "fixtures/include_top.sv",
        "tests/include_config.toml",
        &["include_child.sv", "decl.no_unused_var"],
    );
}
