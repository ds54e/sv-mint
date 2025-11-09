use assert_cmd::prelude::*;
use predicates::str::contains;
use std::fs::{self};
use std::path::{Path, PathBuf};
use std::process::Command;
use tempfile::TempDir;

fn write_file(p: impl Into<PathBuf>, s: &str) {
    fs::write(p.into(), s).unwrap();
}

fn write_bin(p: impl Into<PathBuf>, b: &[u8]) {
    fs::write(p.into(), b).unwrap();
}

fn make_plugin(dir: &Path, body: &str) -> PathBuf {
    let p = dir.join("plugins");
    fs::create_dir_all(&p).unwrap();
    let path = p.join("define_naming.py");
    write_file(&path, body);
    path
}

fn write_config(dir: &Path, plugin: &str, timeout_ms: u64) {
    let toml = format!(
        r#"[defaults]
timeout_ms_per_file = {timeout_ms}

[plugin]
cmd  = "py"
args = ["-3","-u",'{plugin}']
"#,
        plugin = plugin
    );
    write_file(dir.join("sv-mint.toml"), &toml);
}

fn cargo_cmd_in(cwd: &Path) -> Command {
    let bin_path = assert_cmd::cargo::cargo_bin!("sv-mint");
    let mut cmd = Command::new(bin_path);
    cmd.current_dir(cwd);
    cmd
}

fn run_with_cfg_and_input(cwd: &Path, cfg: impl AsRef<Path>, input: impl AsRef<Path>) -> assert_cmd::assert::Assert {
    let mut cmd = cargo_cmd_in(cwd);
    cmd.arg("--config").arg(cfg.as_ref()).arg(input.as_ref());
    cmd.assert()
}

fn run_in_tempdir_with_input(td: &TempDir, input_rel: &str) -> assert_cmd::assert::Assert {
    run_with_cfg_and_input(td.path(), td.path().join("sv-mint.toml"), td.path().join(input_rel))
}

const DEFINE_UPPER_PLUGIN: &str = r#"#!/usr/bin/env python3
import sys, json, re
pat = re.compile(r'^\s*`define\s+([A-Za-z_][A-Za-z0-9_]*)')
def main():
    try:
        req = json.loads(sys.stdin.read())
    except Exception:
        return
    text = req.get("text", "")
    vs = []
    for i, line in enumerate(text.splitlines(), 1):
        m = pat.match(line)
        if m and not m.group(1).isupper():
            vs.append({
                "rule_id": "naming.define_upper",
                "severity": "error",
                "message": "`define '{}' should be UPPER_CASE".format(m.group(1)),
                "location": {"line": i, "col": 1, "end_line": i, "end_col": len(line)}
            })
    sys.stdout.write(json.dumps({"type":"ViolationsSingle","violations":vs})+"\n")
    sys.stdout.flush()
if __name__ == "__main__":
    main()
"#;

#[test]
fn ok_no_violation() {
    let td = TempDir::new().unwrap();
    let plugin = make_plugin(td.path(), DEFINE_UPPER_PLUGIN);
    write_config(td.path(), plugin.to_str().unwrap(), 3000);
    write_file(td.path().join("in.sv"), "module m; endmodule\n");
    run_in_tempdir_with_input(&td, "in.sv").success().code(0);
}

#[test]
fn violation_reports_exit2_and_message_on_stdout() {
    let td = TempDir::new().unwrap();
    let plugin = make_plugin(td.path(), DEFINE_UPPER_PLUGIN);
    write_config(td.path(), plugin.to_str().unwrap(), 3000);
    write_file(td.path().join("in.sv"), "`define foo 1\n");
    run_in_tempdir_with_input(&td, "in.sv")
        .failure()
        .code(2)
        .stdout(contains(":1:1: [error] naming.define_upper"));
}

#[test]
fn config_not_found_flag_is_error_on_stderr() {
    let td = TempDir::new().unwrap();
    write_file(td.path().join("in.sv"), "module m; endmodule\n");
    run_with_cfg_and_input(
        td.path(),
        td.path().join("missing.toml"),
        td.path().join("in.sv"),
    )
    .failure()
    .code(3)
    .stderr(contains("config not found (from --config)"));
}

#[test]
fn invalid_toml_is_error_on_stderr() {
    let td = TempDir::new().unwrap();
    write_file(td.path().join("sv-mint.toml"), "this = is = not = toml");
    write_file(td.path().join("in.sv"), "module m; endmodule\n");
    run_in_tempdir_with_input(&td, "in.sv")
        .failure()
        .code(3)
        .stderr(contains("invalid toml"));
}

#[test]
fn invalid_response_json_is_error_on_stderr() {
    let td = TempDir::new().unwrap();
    let plugin = make_plugin(
        td.path(),
        r#"#!/usr/bin/env python3
import sys
sys.stdout.write("{bad json}\n"); sys.stdout.flush()
"#,
    );
    write_config(td.path(), plugin.to_str().unwrap(), 3000);
    write_file(td.path().join("in.sv"), "module m; endmodule\n");
    run_in_tempdir_with_input(&td, "in.sv")
        .failure()
        .code(3)
        .stderr(contains("invalid response json"));
}

#[test]
fn wrong_type_is_protocol_error_on_stderr() {
    let td = TempDir::new().unwrap();
    let plugin = make_plugin(
        td.path(),
        r#"#!/usr/bin/env python3
import sys, json
sys.stdout.write(json.dumps({"type":"X","violations":[]})+"\n")
sys.stdout.flush()
"#,
    );
    write_config(td.path(), plugin.to_str().unwrap(), 3000);
    write_file(td.path().join("in.sv"), "module m; endmodule\n");
    run_in_tempdir_with_input(&td, "in.sv")
        .failure()
        .code(3)
        .stderr(contains("protocol error"));
}

#[test]
fn timeout_is_error_on_stderr() {
    let td = TempDir::new().unwrap();
    let plugin = make_plugin(
        td.path(),
        r#"#!/usr/bin/env python3
import time; time.sleep(10)
"#,
    );
    write_config(td.path(), plugin.to_str().unwrap(), 100);
    write_file(td.path().join("in.sv"), "module m; endmodule\n");
    run_in_tempdir_with_input(&td, "in.sv")
        .failure()
        .code(3)
        .stderr(contains("timeout"));
}

#[test]
fn size_over_is_error_on_stderr() {
    let td = TempDir::new().unwrap();
    let plugin = make_plugin(td.path(), DEFINE_UPPER_PLUGIN);
    write_config(td.path(), plugin.to_str().unwrap(), 3000);
    let mut s = String::new();
    s.reserve(8 * 1024 * 1024 + 1);
    s.push_str("module m;\n");
    s.push_str(&"a".repeat(8 * 1024 * 1024));
    write_file(td.path().join("in.sv"), &s);
    run_in_tempdir_with_input(&td, "in.sv")
        .failure()
        .code(3)
        .stderr(contains("size over"));
}

#[test]
fn bom_is_ignored_and_lineno_ok() {
    let td = TempDir::new().unwrap();
    let plugin = make_plugin(td.path(), DEFINE_UPPER_PLUGIN);
    write_config(td.path(), plugin.to_str().unwrap(), 3000);
    write_file(td.path().join("in.sv"), "\u{FEFF}module m; endmodule\n");
    run_in_tempdir_with_input(&td, "in.sv").success().code(0);
}

#[test]
fn cr_crlf_lf_line_mapping_ok() {
    let td = TempDir::new().unwrap();
    let plugin = make_plugin(td.path(), DEFINE_UPPER_PLUGIN);
    write_config(td.path(), plugin.to_str().unwrap(), 3000);
    let src = "module m;\r`define foo 1\r\nendmodule\n";
    write_file(td.path().join("in.sv"), src);
    run_in_tempdir_with_input(&td, "in.sv")
        .failure()
        .code(2)
        .stdout(contains(":2:1: [error]"));
}

#[test]
fn invalid_utf8_is_error_on_stderr() {
    let td = TempDir::new().unwrap();
    let plugin = make_plugin(td.path(), DEFINE_UPPER_PLUGIN);
    write_config(td.path(), plugin.to_str().unwrap(), 3000);
    write_bin(td.path().join("in.sv"), &[0x61, 0x80, 0x62, 0x0A]);
    run_in_tempdir_with_input(&td, "in.sv")
        .failure()
        .code(3)
        .stderr(contains("invalid utf-8"));
}
