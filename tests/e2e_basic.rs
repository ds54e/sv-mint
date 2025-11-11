use sv_mint::config::{Config, Defaults, LoggingConfig, Plugin, Stages};
use sv_mint::core::pipeline::Pipeline;
use sv_mint::svparser::SvParserCfg;
use sv_mint::types::Stage;

fn make_cfg(enabled: Vec<Stage>) -> Config {
    Config {
        logging: LoggingConfig::default(),
        defaults: Defaults {
            timeout_ms_per_file: 3000,
        },
        plugin: Plugin {
            cmd: "py".to_string(),
            args: vec![],
        },
        stages: Stages { enabled },
        svparser: SvParserCfg::default(),
        rules: serde_json::Value::Null,
    }
}

#[test]
fn pipeline_single_file_no_violations() {
    let tmp = std::env::temp_dir().join(format!("svmint_test_single_{}", std::process::id()));
    let _ = std::fs::create_dir_all(&tmp);
    let f = tmp.join("a.sv");
    std::fs::write(&f, "module m; endmodule\n").expect("write file");

    let cfg = make_cfg(vec![Stage::RawText, Stage::PpText, Stage::Cst, Stage::Ast]);
    let p = Pipeline::new(&cfg);
    let summary = p.run_files(&[f.clone()]);

    assert!(!summary.had_error, "had_error should be false");
    assert_eq!(summary.violations, 0, "violations should be zero");
}

#[test]
fn pipeline_multiple_files_aggregates() {
    let tmp = std::env::temp_dir().join(format!("svmint_test_multi_{}", std::process::id()));
    let _ = std::fs::create_dir_all(&tmp);
    let f1 = tmp.join("a.sv");
    let f2 = tmp.join("b.sv");
    std::fs::write(&f1, "module a; endmodule\n").expect("write a");
    std::fs::write(&f2, "module b; endmodule\n").expect("write b");

    let cfg = make_cfg(vec![Stage::RawText, Stage::PpText]);
    let p = Pipeline::new(&cfg);
    let summary = p.run_files(&[f1.clone(), f2.clone()]);

    assert!(!summary.had_error, "had_error should be false");
    assert_eq!(summary.violations, 0, "violations should be zero");
}
