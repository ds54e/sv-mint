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
    run_fixture("fixtures/unused_net_violation.sv", "nets_not_left_unused");
}

#[test]
fn allows_marked_unused_net() {
    run_fixture_success("fixtures/unused_net_compliant.sv");
}

#[test]
fn detects_unused_param_violation() {
    run_fixture("fixtures/unused_param_violation.sv", "params_not_left_unused");
}

#[test]
fn allows_marked_unused_param() {
    run_fixture_success("fixtures/unused_param_compliant.sv");
}

#[test]
fn detects_unused_var_violation() {
    run_fixture("fixtures/unused_var_violation.sv", "vars_not_left_unused");
}

#[test]
fn allows_marked_unused_var() {
    run_fixture_success("fixtures/unused_var_compliant.sv");
}

#[test]
fn detects_unused_port_violation() {
    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("sv-mint"));
    cmd.arg("--disable").arg("instances_use_named_ports");
    cmd.arg("fixtures/unused_port_violation.sv");
    cmd.assert().failure().stdout(contains("ports_not_left_unused"));
}

#[test]
fn allows_used_ports() {
    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("sv-mint"));
    cmd.arg("--disable").arg("instances_use_named_ports");
    cmd.arg("fixtures/unused_port_compliant.sv");
    cmd.assert().success();
}

#[test]
fn allows_unused_port_with_unused_comment() {
    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("sv-mint"));
    cmd.arg("--disable").arg("instances_use_named_ports");
    cmd.arg("fixtures/unused_port_unused_comment.sv");
    cmd.assert().success();
}

#[test]
fn detects_multiple_nonblocking_assignments() {
    run_fixture("fixtures/multiple_nonblocking.sv", "instances_use_named_ports");
}

#[test]
fn detects_bare_always() {
    run_fixture("fixtures/always_plain.sv", "always_is_structured");
    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("sv-mint"));
    cmd.arg("--disable").arg("instances_use_named_ports");
    cmd.arg("fixtures/always_structured_ok.sv");
    cmd.assert().success();
}

#[test]
fn detects_sensitivity_or() {
    run_fixture("fixtures/sensitivity_or_violation.sv", "sensitivity_list_uses_commas");
}

#[test]
fn allows_sensitivity_commas() {
    run_fixture_success("fixtures/sensitivity_comma_ok.sv");
}

#[test]
fn detects_net_naming_violations() {
    run_fixture("fixtures/net_lower_snake_violation.sv", "net_names_lower_snake");
    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("sv-mint"));
    cmd.arg("--disable").arg("nets_not_left_unused");
    cmd.arg("fixtures/net_lower_snake_ok.sv");
    cmd.assert().success();
}

#[test]
fn detects_var_naming_violations() {
    run_fixture("fixtures/var_lower_snake_violation.sv", "var_names_lower_snake");
    run_fixture_success("fixtures/var_lower_snake_ok.sv");
}

#[test]
fn detects_parameter_naming_violations() {
    run_fixture("fixtures/parameter_case_violation.sv", "parameter_names_uppercase");
    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("sv-mint"));
    cmd.arg("--disable").arg("params_not_left_unused");
    cmd.arg("fixtures/parameter_case_ok.sv");
    cmd.assert().success();
}

#[test]
fn detects_parameter_missing_type() {
    run_fixture("fixtures/parameter_missing_type.sv", "parameter_has_type");
}

#[test]
fn detects_parameter_range_only_type() {
    run_fixture("fixtures/parameter_range_only_violation.sv", "parameter_has_type");
}

#[test]
fn detects_localparam_missing_type() {
    run_fixture("fixtures/localparam_missing_type.sv", "parameter_has_type");
}

#[test]
fn allows_parameter_with_type() {
    run_fixture_success("fixtures/parameter_with_type.sv");
}

#[test]
fn detects_localparam_naming_violations() {
    run_fixture("fixtures/localparam_case_violation.sv", "localparam_names_uppercase");
    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("sv-mint"));
    cmd.arg("--disable").arg("params_not_left_unused");
    cmd.arg("fixtures/localparam_case_ok.sv");
    cmd.assert().success();
}

#[test]
fn detects_multiple_modules() {
    run_fixture("fixtures/multiple_modules_violation.sv", "one_module_per_file");
    run_fixture_success("fixtures/multiple_modules_ok.sv");
}

#[test]
fn detects_filename_mismatch() {
    run_fixture("fixtures/module_filename_mismatch.sv", "module_name_matches_filename");
    run_fixture_success("fixtures/module_filename_match_ok.sv");
}

#[test]
fn allows_unique_case_without_default() {
    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("sv-mint"));
    cmd.arg("--disable").arg("instances_use_named_ports");
    cmd.arg("fixtures/case_missing_default_unique_ok.sv");
    cmd.assert().success();
}

#[test]
fn detects_module_inst_violations() {
    run_fixture("fixtures/module_inst_violation.sv", "instances_use_named_ports");
}

#[test]
fn detects_typedef_violations() {
    run_fixture("fixtures/typedef_violation.sv", "enum_type_names_lower_snake_e");
}

#[test]
fn detects_function_scope_violations() {
    run_fixture(
        "fixtures/function_scope_violation.sv",
        "functions_marked_automatic_or_static",
    );
}

#[test]
fn detects_function_missing_types() {
    run_fixture("fixtures/function_missing_types.sv", "functions_have_explicit_types");
}

#[test]
fn allows_function_with_types() {
    run_fixture_success("fixtures/function_with_explicit_types.sv");
}

#[test]
fn detects_macro_undef_violations() {
    run_fixture("fixtures/macro_violation.sv", "macros_close_with_undef");
}

#[test]
fn detects_define_upper_violations() {
    run_fixture("fixtures/macro_define_upper_violation.sv", "macro_names_uppercase");
    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("sv-mint"));
    cmd.arg("--disable").arg("macros_not_unused");
    cmd.arg("--disable").arg("macros_close_with_undef");
    cmd.arg("fixtures/macro_define_upper_ok.sv");
    cmd.assert().success();
}

#[test]
fn detects_unused_macro() {
    run_fixture("fixtures/macro_unused.sv", "macros_not_unused");
    run_fixture_success("fixtures/macro_used.sv");
}

#[test]
fn reports_include_file_path() {
    run_with_config(
        "fixtures/include_top.sv",
        "tests/include_config.toml",
        &["include_child.sv", "vars_not_left_unused"],
    );
}
