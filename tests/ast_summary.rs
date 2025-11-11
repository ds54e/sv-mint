use std::fs;
use sv_mint::svparser::{SvDriver, SvParserCfg};
use tempfile::tempdir;

#[test]
fn ast_summary_extracts_module_and_params() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("t.sv");
    let src = r#"module top #(parameter WIDTH=8, localparam DEPTH=16) (); endmodule"#;
    fs::write(&path, src).unwrap();

    let cfg = SvParserCfg {
        include_paths: vec![],
        defines: vec![],
        strip_comments: true,
        ignore_include: true,
        allow_incomplete: true,
    };
    let d = SvDriver::new(&cfg);
    let arts = d.parse_text(src, &path);
    assert!(arts.has_cst);

    let mut mods = Vec::new();
    let mut params = Vec::new();
    for v in arts.ast.decls {
        if let Some(k) = v.get("kind").and_then(|x| x.as_str()) {
            if k == "module" {
                if let Some(n) = v.get("name").and_then(|x| x.as_str()) {
                    mods.push(n.to_string());
                }
            } else if k == "param" {
                if let Some(n) = v.get("name").and_then(|x| x.as_str()) {
                    params.push(n.to_string());
                }
            }
        }
    }
    assert!(mods.contains(&"top".to_string()));
    assert!(params.contains(&"WIDTH".to_string()));
    assert!(params.contains(&"DEPTH".to_string()));
}
