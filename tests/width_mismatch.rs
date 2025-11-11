#[cfg(windows)]
mod width_mismatch_e2e {
    use serde_json::json;
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};
    use sv_mint::config::{Config, Defaults, Plugin, Stages};
    use sv_mint::core::pipeline::Pipeline;
    use sv_mint::diag::logging;
    use sv_mint::types::Stage;

    fn tmp(prefix: &str) -> std::path::PathBuf {
        let n = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
        std::env::temp_dir().join(format!("{}_{}", prefix, n))
    }

    #[test]
    fn detects_width_mismatch_in_assign() {
        if std::process::Command::new("py").arg("--version").output().is_err() {
            return;
        }
        let base = tmp("svmint_widthmm");
        fs::create_dir_all(&base).unwrap();
        let plugin_path = base.join("rules.py");
        fs::write(&plugin_path, include_str!("width_mismatch_rules_inline.py")).unwrap();

        let sv_path = base.join("t.sv");
        let sv = r#"
module top #(parameter W=8) ();
  logic [W-1:0] y;
  logic [3:0]   x;
  assign y = x;
endmodule
"#;
        fs::write(&sv_path, sv).unwrap();

        let cfg = Config {
            logging: Default::default(),
            defaults: Defaults {
                timeout_ms_per_file: 3000,
            },
            plugin: Plugin {
                cmd: "py".into(),
                args: vec!["-3".into(), "-u".into(), plugin_path.to_string_lossy().into_owned()],
            },
            stages: Stages {
                enabled: vec![Stage::PpText, Stage::Ast],
            },
            svparser: Default::default(),
            rules: json!({}),
        };
        logging::init(&cfg.logging).unwrap();
        let pipeline = Pipeline::new(&cfg);
        let n = pipeline.run_file(&sv_path).unwrap();
        assert!(n >= 1);
    }
}

#[cfg(not(windows))]
#[test]
fn skip_on_non_windows() {
    assert!(true);
}
