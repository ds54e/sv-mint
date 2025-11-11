#[cfg(windows)]
mod plugin_error_paths {
    use serde_json::json;
    use std::fs;
    use std::process::Command;
    use std::time::{SystemTime, UNIX_EPOCH};
    use sv_mint::config::{Config, Defaults, Plugin, Stages};
    use sv_mint::core::pipeline::Pipeline;
    use sv_mint::diag::logging;
    use sv_mint::types::Stage;

    fn py_available() -> bool {
        Command::new("py").arg("--version").output().is_ok()
    }

    fn tmpdir(prefix: &str) -> std::path::PathBuf {
        let n = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
        std::env::temp_dir().join(format!("{}_{}", prefix, n))
    }

    #[test]
    fn plugin_timeout_sets_had_error() {
        if !py_available() {
            return;
        }
        let base = tmpdir("svmint_err_timeout");
        fs::create_dir_all(&base).unwrap();
        let plugin_path = base.join("rules.py");
        let py = r#"import time; time.sleep(2)"#;
        fs::write(&plugin_path, py).unwrap();

        let sv_path = base.join("t.sv");
        fs::write(&sv_path, "module m; endmodule\n").unwrap();

        let cfg = Config {
            logging: Default::default(),
            defaults: Defaults {
                timeout_ms_per_file: 200,
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
        assert!(summary.had_error);
    }

    #[test]
    fn plugin_stdout_too_large_sets_had_error() {
        if !py_available() {
            return;
        }
        let base = tmpdir("svmint_err_stdout");
        fs::create_dir_all(&base).unwrap();
        let plugin_path = base.join("rules.py");
        let py = r#"import sys; sys.stdout.write("X"*(16*1024*1024+1024))"#;
        fs::write(&plugin_path, py).unwrap();

        let sv_path = base.join("t.sv");
        fs::write(&sv_path, "module m; endmodule\n").unwrap();

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
                enabled: vec![Stage::RawText],
            },
            svparser: Default::default(),
            rules: json!({}),
        };
        logging::init(&cfg.logging).unwrap();
        let pipeline = Pipeline::new(&cfg);
        let summary = pipeline.run_files(&[sv_path]).unwrap();
        assert!(summary.had_error);
    }

    #[test]
    fn plugin_stderr_too_large_sets_had_error() {
        if !py_available() {
            return;
        }
        let base = tmpdir("svmint_err_stderr");
        fs::create_dir_all(&base).unwrap();
        let plugin_path = base.join("rules.py");
        let py = r#"import sys; sys.stderr.write("E"*(4*1024*1024+1024)); sys.stdout.write('{"type":"ViolationsStage","stage":"raw_text","violations":[]}')"#;
        fs::write(&plugin_path, py).unwrap();

        let sv_path = base.join("t.sv");
        fs::write(&sv_path, "module m; endmodule\n").unwrap();

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
                enabled: vec![Stage::RawText],
            },
            svparser: Default::default(),
            rules: json!({}),
        };
        logging::init(&cfg.logging).unwrap();
        let pipeline = Pipeline::new(&cfg);
        let summary = pipeline.run_files(&[sv_path]).unwrap();
        assert!(summary.had_error);
    }

    #[test]
    fn plugin_exit_nonzero_sets_had_error() {
        if !py_available() {
            return;
        }
        let base = tmpdir("svmint_err_exit");
        fs::create_dir_all(&base).unwrap();
        let plugin_path = base.join("rules.py");
        let py = r#"import sys, json; sys.stdout.write('{"type":"ViolationsStage","stage":"raw_text","violations":[]}'); sys.exit(2)"#;
        fs::write(&plugin_path, py).unwrap();

        let sv_path = base.join("t.sv");
        fs::write(&sv_path, "module m; endmodule\n").unwrap();

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
                enabled: vec![Stage::RawText],
            },
            svparser: Default::default(),
            rules: json!({}),
        };
        logging::init(&cfg.logging).unwrap();
        let pipeline = Pipeline::new(&cfg);
        let summary = pipeline.run_files(&[sv_path]).unwrap();
        assert!(summary.had_error);
    }
}

#[cfg(not(windows))]
#[test]
fn skip_on_non_windows() {
    assert!(true);
}
