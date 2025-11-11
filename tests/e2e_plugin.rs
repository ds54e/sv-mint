#![cfg_attr(not(windows), allow(unused))]
#[cfg(windows)]
mod windows_e2e {
    use serde_json::json;
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};
    use sv_mint::config::{Config, Defaults, Plugin, Stages};
    use sv_mint::core::pipeline::Pipeline;
    use sv_mint::diag::logging;
    use sv_mint::types::Stage;

    #[test]
    fn basic_violation_flow() {
        let base = std::env::temp_dir().join(format!(
            "svmint_e2e_{}",
            SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos()
        ));
        fs::create_dir_all(&base).unwrap();

        let plugin_path = base.join("rules.py");
        let py = r#"
import sys, json
data = sys.stdin.read()
req = json.loads(data) if data else {}
viol = []
if req.get("stage") == "raw_text":
    payload = req.get("payload") or {}
    txt = payload.get("text") or ""
    if "unused" in txt:
        viol.append({"rule_id":"decl.unused","severity":"warning","message":"declared but never used","location":{"line":1,"col":1,"end_line":1,"end_col":1}})
resp = {"type":"ViolationsStage","stage":req.get("stage"),"violations":viol}
sys.stdout.write(json.dumps(resp))
"#;
        fs::write(&plugin_path, py).unwrap();

        let sv_path = base.join("test.sv");
        fs::write(&sv_path, "module m; // unused\nendmodule\n").unwrap();

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
        let n = pipeline.run_file(&sv_path).unwrap();
        assert_eq!(n, 1);
    }
}

#[cfg(not(windows))]
#[test]
fn skip_on_non_windows() {
    assert!(true);
}
