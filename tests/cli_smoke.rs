use assert_cmd::Command;
use predicates::prelude::{PredicateBooleanExt, PredicateBoxExt};
use predicates::str::contains;

fn run_fixture(path: &str, fragment: &str) {
    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("sv-mint"));
    cmd.arg(path);
    cmd.assert().failure().stdout(contains(fragment));
}

fn run_fixture_with_fragments(path: &str, expected: &[&str]) {
    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("sv-mint"));
    cmd.arg(path);
    let mut pred = contains(expected[0]).boxed();
    for frag in &expected[1..] {
        pred = pred.and(contains(*frag)).boxed();
    }
    cmd.assert().failure().stdout(pred);
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
fn detects_port_wildcard() {
    run_fixture("fixtures/port_wildcard_violation.sv", "module.no_port_wildcard");
}

#[test]
fn detects_naming_violations() {
    run_fixture("fixtures/naming_violations.sv", "naming.module_case");
}

#[test]
fn detects_parameter_violations() {
    run_fixture("fixtures/parameter_violation.sv", "naming.parameter_upper");
}

#[test]
fn detects_port_suffix_violations() {
    run_fixture("fixtures/port_suffix_violation.sv", "naming.port_suffix");
}

#[test]
fn detects_enum_prefix_violations() {
    run_fixture("fixtures/typedef_violation.sv", "typedef.enum_value_prefix");
}

#[test]
fn detects_language_violations() {
    run_fixture("fixtures/lang_violations.sv", "lang.prefer_always_comb");
}

#[test]
fn detects_global_define_violations() {
    run_fixture("fixtures/global_define_violations.sv", "global.prefer_parameters");
}

#[test]
fn detects_width_literal_violations() {
    run_fixture("fixtures/width_literal_violation.sv", "width.unsized_base_literal");
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
fn detects_case_unique_violations() {
    run_fixture("fixtures/case_unique_violation.sv", "lang.case_requires_unique");
}

#[test]
fn allows_unique_case_without_default() {
    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("sv-mint"));
    cmd.arg("--disable").arg("module.named_ports_required");
    cmd.arg("fixtures/case_missing_default_unique_ok.sv");
    cmd.assert().success();
}

#[test]
fn detects_package_mismatch() {
    run_fixture("fixtures/package_mismatch.sv", "package.multiple");
}

#[test]
fn detects_module_inst_violations() {
    run_fixture("fixtures/module_inst_violation.sv", "module.named_ports_required");
}

#[test]
fn detects_header_missing() {
    run_fixture("fixtures/header_missing.sv", "header.missing_spdx");
}

#[test]
fn detects_typedef_violations() {
    run_fixture("fixtures/typedef_violation.sv", "typedef.enum_suffix");
}

#[test]
fn detects_always_ff_violations() {
    run_fixture("fixtures/always_ff_violation.sv", "lang.always_ff_reset");
}

#[test]
fn detects_function_scope_violations() {
    run_fixture("fixtures/function_scope_violation.sv", "style.function_scope");
}

#[test]
fn detects_randomize_macros() {
    run_fixture("fixtures/randomize_violation.sv", "rand.dv_macro_required");
}

#[test]
fn detects_randomize_with_macros() {
    run_fixture("fixtures/randomize_with_violation.sv", "rand.dv_macro_with_required");
}

#[test]
fn detects_logging_violations() {
    run_fixture_with_fragments(
        "fixtures/logging_violation.sv",
        &[
            "log.uvm_arg_macro",
            "log.no_uvm_warning",
            "log.no_uvm_report_api",
            "log.no_display",
            "log.no_none_full",
            "log.allowed_verbosity",
        ],
    );
}

#[test]
fn detects_dpi_prefix_violations() {
    run_fixture_with_fragments("fixtures/dpi_violation.sv", &["dpi.import_prefix", "dpi.export_prefix"]);
}

#[test]
fn detects_macro_undef_violations() {
    run_fixture("fixtures/macro_violation.sv", "macro.missing_undef");
}

#[test]
fn detects_wait_violations() {
    run_fixture_with_fragments(
        "fixtures/wait_violation.sv",
        &["flow.wait_macro_required", "flow.wait_fork_isolation"],
    );
}

#[test]
fn detects_spinwait_violations() {
    run_fixture("fixtures/spinwait_violation.sv", "flow.spinwait_macro_required");
}

#[test]
fn detects_uvm_do_usage() {
    run_fixture("fixtures/uvm_do_violation.sv", "seq.no_uvm_do");
}

#[test]
fn detects_macro_guard_requirements() {
    run_fixture("fixtures/global_macros.svh", "macro.guard_required");
}

#[test]
fn detects_local_macro_guards() {
    run_fixture("fixtures/local_macro_guard_violation.sv", "macro.no_local_guard");
}

#[test]
fn detects_dv_macro_location() {
    run_fixture("fixtures/local_dv_macro.sv", "macro.dv_prefix_header_only");
}

#[test]
fn detects_module_macro_prefix() {
    run_fixture("fixtures/module_macro_violation.sv", "macro.module_prefix");
}

#[test]
fn detects_scoreboard_eot_requirements() {
    run_fixture("fixtures/scoreboard_violation.sv", "scoreboard.dv_eot_required");
}

#[test]
fn detects_program_constructs() {
    run_fixture("fixtures/program_violation.sv", "lang.no_program_construct");
}

#[test]
fn detects_comparison_checks() {
    run_fixture("fixtures/comparison_violation.sv", "check.dv_macro_required");
}

#[test]
fn detects_missing_default_nettype() {
    run_fixture("fixtures/default_nettype_missing.sv", "lang.default_nettype_missing");
}

#[test]
fn detects_default_nettype_value() {
    run_fixture("fixtures/default_nettype_wire.sv", "lang.default_nettype_none");
}

#[test]
fn detects_default_nettype_placement() {
    run_fixture_with_fragments(
        "fixtures/default_nettype_late.sv",
        &["lang.default_nettype_placement", "lang.default_nettype_reset"],
    );
}

#[test]
fn detects_fork_label_usage() {
    run_fixture_with_fragments(
        "fixtures/fork_label_violation.sv",
        &["flow.no_fork_label", "flow.no_disable_fork_label"],
    );
}

#[test]
fn reports_include_file_path() {
    run_with_config(
        "fixtures/include_top.sv",
        "tests/include_config.toml",
        &["include_child.sv", "decl.unused_var"],
    );
}
