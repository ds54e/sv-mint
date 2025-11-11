use std::fs;
use sv_mint::svparser::{SvDriver, SvParserCfg};
use tempfile::tempdir;

#[test]
fn refs_extract_from_expr_and_ports() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("t.sv");
    let src = r#"
module top #(parameter WIDTH=8) (input logic a, output logic y);
    logic tmp;
    assign tmp = a & WIDTH;
    sub u0(.i(tmp), .o(y));
endmodule
module sub(input logic i, output logic o);
    assign o = i;
endmodule
"#;
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
    let names: Vec<String> = arts
        .ast
        .refs
        .iter()
        .filter_map(|v| v.get("name").and_then(|x| x.as_str()).map(|s| s.to_string()))
        .collect();
    assert!(names.iter().any(|n| n == "a"));
    assert!(names.iter().any(|n| n == "WIDTH"));
    assert!(names.iter().any(|n| n == "tmp"));
}
