use assert_cmd::Command;
use std::path::Path;
use std::sync::OnceLock;
use sv_mint::config::load_from_path;

static RULE_IDS: OnceLock<Vec<String>> = OnceLock::new();

fn rule_ids() -> &'static [String] {
    RULE_IDS.get_or_init(|| {
        let (cfg, _) = load_from_path(None).expect("load config");
        cfg.rule.into_iter().map(|r| r.id).collect()
    })
}

fn fixture_rule_id(path: &str) -> String {
    let p = Path::new(path);
    let parent = p.parent().and_then(|d| d.file_name()).and_then(|s| s.to_str());
    if let Some(dir) = parent {
        if rule_ids().iter().any(|id| id == dir) {
            return dir.to_string();
        }
    }
    let name = p
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or(path);
    let trimmed = name.trim_end_matches(".sv");
    if let Some(found) = rule_ids()
        .iter()
        .filter(|id| trimmed.starts_with(id.as_str()))
        .max_by_key(|id| id.len())
    {
        return found.clone();
    }
    trimmed.to_string()
}

fn expect_fail(path: &str, fragment: &str) {
    let rule_id = fixture_rule_id(path);
    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("sv-mint"));
    cmd.arg("--only").arg(&rule_id);
    cmd.arg(path);
    let out = cmd.output().expect("failed to run sv-mint");
    let code = out.status.code();
    let stdout = String::from_utf8_lossy(&out.stdout);
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        code == Some(2),
        "expected exit 2 for rule {} on {}, got {:?}\nstdout:\n{}\nstderr:\n{}",
        rule_id,
        path,
        code,
        stdout,
        stderr
    );
    assert!(
        stdout.contains(fragment),
        "expected stdout to contain '{}' for rule {} on {}\nstdout:\n{}\nstderr:\n{}",
        fragment,
        rule_id,
        path,
        stdout,
        stderr
    );
}

fn expect_pass(path: &str) {
    let rule_id = fixture_rule_id(path);
    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("sv-mint"));
    cmd.arg("--only").arg(&rule_id);
    cmd.arg(path);
    let out = cmd.output().expect("failed to run sv-mint");
    let stdout = String::from_utf8_lossy(&out.stdout);
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        out.status.success(),
        "expected success for rule {} on {}\nstdout:\n{}\nstderr:\n{}",
        rule_id,
        path,
        stdout,
        stderr
    );
}

fn run_with_config(path: &str, config: &str, expected: &[&str]) {
    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("sv-mint"));
    cmd.arg("--config").arg(config).arg(path);
    let out = cmd.output().expect("failed to run sv-mint");
    let code = out.status.code();
    let stdout = String::from_utf8_lossy(&out.stdout);
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        code == Some(2),
        "expected exit 2 on {}, got {:?}\nstdout:\n{}\nstderr:\n{}",
        path,
        code,
        stdout,
        stderr
    );
    for frag in expected {
        assert!(
            stdout.contains(frag),
            "expected stdout to contain '{}' on {}\nstdout:\n{}\nstderr:\n{}",
            frag,
            path,
            stdout,
            stderr
        );
    }
}

include!(concat!(env!("OUT_DIR"), "/cli_fixtures.rs"));

#[test]
fn detects_module_instantiations_includes() {
    run_with_config(
        "fixtures/cli/include_top.sv",
        "tests/include_config.toml",
        &["include_child.sv", "vars_not_left_unused"],
    );
}
