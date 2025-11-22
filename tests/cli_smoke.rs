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
fn detects_nets_not_left_unused() {
    run_fixture("fixtures/rules/nets_not_left_unused_bad.sv", "nets_not_left_unused");
}

#[test]
fn allows_unused_net_marked() {
    run_fixture_success("fixtures/rules/nets_not_left_unused_good.sv");
}

#[test]
fn detects_params_not_left_unused() {
    run_fixture("fixtures/rules/params_not_left_unused_bad.sv", "params_not_left_unused");
}

#[test]
fn allows_unused_param_marked() {
    run_fixture_success("fixtures/rules/params_not_left_unused_good.sv");
}

#[test]
fn detects_vars_not_left_unused() {
    run_fixture("fixtures/rules/vars_not_left_unused_bad.sv", "vars_not_left_unused");
}

#[test]
fn allows_unused_var_marked() {
    run_fixture_success("fixtures/rules/vars_not_left_unused_good.sv");
}

#[test]
fn detects_ports_not_left_unused() {
    run_fixture("fixtures/rules/ports_not_left_unused_bad.sv", "ports_not_left_unused");
}

#[test]
fn allows_ports_not_left_unused() {
    run_fixture_success("fixtures/rules/ports_not_left_unused_good.sv");
}

#[test]
fn allows_ports_not_left_unused_with_comment() {
    run_fixture_success("fixtures/rules/ports_not_left_unused_comment_good.sv");
}

#[test]
fn detects_default_nettype_missing_none() {
    run_fixture(
        "fixtures/rules/default_nettype_begins_with_none_bad.sv",
        "default_nettype_begins_with_none",
    );
}

#[test]
fn allows_default_nettype_declared() {
    run_fixture_success("fixtures/rules/default_nettype_begins_with_none_good.sv");
}

#[test]
fn detects_default_nettype_missing_wire_reset() {
    run_fixture(
        "fixtures/rules/default_nettype_ends_with_wire_bad.sv",
        "default_nettype_ends_with_wire",
    );
}

#[test]
fn allows_default_nettype_reset_to_wire() {
    run_fixture_success("fixtures/rules/default_nettype_ends_with_wire_good.sv");
}

#[test]
fn detects_always_is_structured() {
    run_fixture("fixtures/rules/always_is_structured_bad.sv", "always_is_structured");
    run_fixture_success("fixtures/rules/always_is_structured_good.sv");
}

#[test]
fn detects_always_comb_blocking_violation() {
    run_fixture(
        "fixtures/rules/always_comb_uses_blocking_bad.sv",
        "always_comb_uses_blocking",
    );
    run_fixture_success("fixtures/rules/always_comb_uses_blocking_good.sv");
}

#[test]
fn detects_always_ff_blocking_violation() {
    run_fixture(
        "fixtures/rules/always_ff_uses_nonblocking_bad.sv",
        "always_ff_uses_nonblocking",
    );
    run_fixture_success("fixtures/rules/always_ff_uses_nonblocking_good.sv");
}

#[test]
fn detects_case_missing_default() {
    run_fixture(
        "fixtures/rules/case_has_default_branch_bad.sv",
        "case_has_default_branch",
    );
    run_fixture_success("fixtures/rules/case_has_default_branch_good.sv");
}

#[test]
fn detects_sensitivity_or() {
    run_fixture(
        "fixtures/rules/sensitivity_list_uses_commas_bad.sv",
        "sensitivity_list_uses_commas",
    );
    run_fixture_success("fixtures/rules/sensitivity_list_uses_commas_good.sv");
}

#[test]
fn detects_net_naming_violations() {
    run_fixture("fixtures/rules/net_names_lower_snake_bad.sv", "net_names_lower_snake");
    run_fixture_success("fixtures/rules/net_names_lower_snake_good.sv");
}

#[test]
fn detects_var_naming_violations() {
    run_fixture("fixtures/rules/var_names_lower_snake_bad.sv", "var_names_lower_snake");
    run_fixture_success("fixtures/rules/var_names_lower_snake_good.sv");
}

#[test]
fn detects_parameter_naming_violations() {
    run_fixture(
        "fixtures/rules/parameter_names_uppercase_bad.sv",
        "parameter_names_uppercase",
    );
    run_fixture_success("fixtures/rules/parameter_names_uppercase_good.sv");
}

#[test]
fn detects_parameter_missing_type() {
    run_fixture(
        "fixtures/rules/parameter_has_type_missing_type_bad.sv",
        "parameter_has_type",
    );
    run_fixture(
        "fixtures/rules/parameter_has_type_range_only_bad.sv",
        "parameter_has_type",
    );
    run_fixture(
        "fixtures/rules/parameter_has_type_localparam_bad.sv",
        "parameter_has_type",
    );
    run_fixture_success("fixtures/rules/parameter_has_type_good.sv");
}

#[test]
fn detects_localparam_naming_violations() {
    run_fixture(
        "fixtures/rules/localparam_names_uppercase_bad.sv",
        "localparam_names_uppercase",
    );
    run_fixture_success("fixtures/rules/localparam_names_uppercase_good.sv");
}

#[test]
fn detects_multiple_modules() {
    run_fixture("fixtures/rules/one_module_per_file_bad.sv", "one_module_per_file");
    run_fixture_success("fixtures/rules/one_module_per_file_good.sv");
}

#[test]
fn detects_filename_mismatch() {
    run_fixture(
        "fixtures/rules/module_name_matches_filename_bad.sv",
        "module_name_matches_filename",
    );
    run_fixture_success("fixtures/rules/module_name_matches_filename_good.sv");
}

#[test]
fn detects_module_name_case_violation() {
    run_fixture(
        "fixtures/rules/module_names_lower_snake_bad.sv",
        "module_names_lower_snake",
    );
    run_fixture_success("fixtures/rules/module_names_lower_snake_good.sv");
}

#[test]
fn detects_instances_use_named_ports() {
    run_fixture(
        "fixtures/rules/instances_use_named_ports_bad.sv",
        "instances_use_named_ports",
    );
    run_fixture_success("fixtures/rules/instances_use_named_ports_good.sv");
}

#[test]
fn detects_enum_type_name_violation() {
    run_fixture(
        "fixtures/rules/enum_type_names_lower_snake_e_bad.sv",
        "enum_type_names_lower_snake_e",
    );
    run_fixture_success("fixtures/rules/enum_type_names_lower_snake_e_good.sv");
}

#[test]
fn detects_enum_value_case_violation() {
    run_fixture("fixtures/rules/enum_values_uppercase_bad.sv", "enum_values_uppercase");
    run_fixture_success("fixtures/rules/enum_values_uppercase_good.sv");
}

#[test]
fn detects_function_scope_violations() {
    run_fixture(
        "fixtures/rules/functions_marked_automatic_or_static_bad.sv",
        "functions_marked_automatic_or_static",
    );
    run_fixture_success("fixtures/rules/functions_marked_automatic_or_static_good.sv");
}

#[test]
fn detects_function_missing_types() {
    run_fixture(
        "fixtures/rules/functions_have_explicit_types_bad.sv",
        "functions_have_explicit_types",
    );
    run_fixture_success("fixtures/rules/functions_have_explicit_types_good.sv");
}

#[test]
fn detects_macro_undef_violations() {
    run_fixture("fixtures/rules/macros_close_with_undef_bad.sv", "macros_close_with_undef");
    run_fixture_success("fixtures/rules/macros_close_with_undef_good.sv");
}

#[test]
fn detects_macro_prefix_violation() {
    run_fixture(
        "fixtures/rules/macros_use_module_prefix_bad.sv",
        "macros_use_module_prefix",
    );
    run_fixture_success("fixtures/rules/macros_use_module_prefix_good.sv");
}

#[test]
fn detects_define_upper_violations() {
    run_fixture("fixtures/rules/macro_names_uppercase_bad.sv", "macro_names_uppercase");
    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("sv-mint"));
    cmd.arg("--disable").arg("macros_not_unused");
    cmd.arg("--disable").arg("macros_close_with_undef");
    cmd.arg("fixtures/rules/macro_names_uppercase_good.sv");
    cmd.assert().success();
}

#[test]
fn detects_unused_macro() {
    run_fixture("fixtures/rules/macros_not_unused_bad.sv", "macros_not_unused");
    run_fixture_success("fixtures/rules/macros_not_unused_good.sv");
}

#[test]
fn detects_disable_targets_fork_only() {
    run_fixture(
        "fixtures/rules/disable_targets_fork_only_bad.sv",
        "disable_targets_fork_only",
    );
    run_fixture_success("fixtures/rules/disable_targets_fork_only_good.sv");
}

#[test]
fn detects_module_instantiations_includes() {
    run_with_config(
        "fixtures/cli/include_top.sv",
        "tests/include_config.toml",
        &["include_child.sv", "vars_not_left_unused"],
    );
}

#[test]
fn detects_no_define_inside_package() {
    run_fixture(
        "fixtures/rules/no_define_inside_package_bad.sv",
        "no_define_inside_package",
    );
    run_fixture_success("fixtures/rules/no_define_inside_package_good.sv");
}

#[test]
fn detects_one_package_per_file() {
    run_fixture("fixtures/rules/one_package_per_file_bad.sv", "one_package_per_file");
    run_fixture_success("fixtures/rules/one_package_per_file_good.sv");
}

#[test]
fn detects_port_name_lower_snake() {
    run_fixture("fixtures/rules/port_names_lower_snake_bad.sv", "port_names_lower_snake");
    run_fixture_success("fixtures/rules/port_names_lower_snake_good.sv");
}

#[test]
fn detects_port_direction_suffix() {
    run_fixture(
        "fixtures/rules/port_names_have_direction_suffix_bad.sv",
        "port_names_have_direction_suffix",
    );
    run_fixture_success("fixtures/rules/port_names_have_direction_suffix_good.sv");
}

#[test]
fn detects_typedef_lower_snake_t() {
    run_fixture(
        "fixtures/rules/typedef_names_lower_snake_t_bad.sv",
        "typedef_names_lower_snake_t",
    );
    run_fixture_success("fixtures/rules/typedef_names_lower_snake_t_good.sv");
}
