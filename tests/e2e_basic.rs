#[cfg(windows)]
mod e2e_basic {
    use serde_json::json;
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};
    use sv_mint::config::{Config, Defaults, Plugin, Stages};
    use sv_mint::core::pipeline::Pipeline;
    use sv_mint::diag::logging;
    use sv_mint::types::Stage;

    fn unique_dir(prefix: &str) -> std::path::PathBuf {
        let n = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
        std::env::temp_dir().join(format!("{}_{}", prefix, n))
    }

    #[test]
    fn run_files_no_violation() {
        if std::process::Command::new("py").arg("--version").output().is_err() {
            return;
        }
        let base = unique_dir("svmint_e2e_noviol");
        fs::create_dir_all(&base).unwrap();
        let plugin_path = base.join("rules.py");
        let py = r#"import sys, json
data = sys.stdin.buffer.read()
req = json.loads(data.decode("utf-8")) if data else {}
resp = {"type":"ViolationsStage","stage":req.get("stage"),"violations":[]}
sys.stdout.write(json.dumps(resp))"#;
        fs::write(&plugin_path, py).unwrap();
        let sv_path = base.join("test.sv");
        fs::write(&sv_path, "module m; endmodule\n").unwrap();
        let cfg = Config {
            logging: Default::default(),
            defaults: Defaults {
                timeout_ms_per_file: 5000,
            },
            plugin: Plugin {
                cmd: "py".into(),
                args: vec!["-3".into(), "-u".into(), plugin_path.to_string_lossy().into_owned()],
            },
            stages: Stages {
                enabled: vec![Stage::RawText],
            },
            svparser: Default::default(),
            rules: json!({}),
        };
        logging::init(&cfg.logging).unwrap();
        let pipeline = Pipeline::new(&cfg);
        let summary = pipeline.run_files(&[sv_path]).unwrap();
        assert!(!summary.had_error);
        assert_eq!(summary.violations, 0);
    }

    #[test]
    fn run_files_with_violation() {
        if std::process::Command::new("py").arg("--version").output().is_err() {
            return;
        }
        let base = unique_dir("svmint_e2e_viol");
        fs::create_dir_all(&base).unwrap();
        let plugin_path = base.join("rules.py");
        let py = r#"import sys, json
data = sys.stdin.buffer.read()
req = json.loads(data.decode("utf-8")) if data else {}
viol = []
if req.get("stage") == "raw_text":
    payload = req.get("payload") or {}
    txt = payload.get("text") or ""
    if "unused" in txt.lower():
        viol.append({"rule_id":"decl.unused","severity":"warning","message":"declared but never used","location":{"line":1,"col":1,"end_line":1,"end_col":1}})
resp = {"type":"ViolationsStage","stage":req.get("stage"),"violations":viol}
sys.stdout.write(json.dumps(resp))"#;
        fs::write(&plugin_path, py).unwrap();
        let sv_path = base.join("test.sv");
        fs::write(&sv_path, "module m; // UNUSED\nendmodule\n").unwrap();
        let cfg = Config {
            logging: Default::default(),
            defaults: Defaults {
                timeout_ms_per_file: 5000,
            },
            plugin: Plugin {
                cmd: "py".into(),
                args: vec!["-3".into(), "-u".into(), plugin_path.to_string_lossy().into_owned()],
            },
            stages: Stages {
                enabled: vec![Stage::RawText],
            },
            svparser: Default::default(),
            rules: json!({}),
        };
        logging::init(&cfg.logging).unwrap();
        let pipeline = Pipeline::new(&cfg);
        let summary = pipeline.run_files(&[sv_path]).unwrap();
        assert!(!summary.had_error);
        assert!(summary.violations >= 1);
    }
}

#[cfg(not(windows))]
#[test]
fn skip_on_non_windows() {
    assert!(true);
}
