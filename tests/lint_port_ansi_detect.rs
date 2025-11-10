use std::fs::{create_dir_all, write};
use std::path::PathBuf;
use std::process::Command;

use assert_cmd::prelude::*;
use predicates::prelude::*;
use tempfile::tempdir;

fn bin() -> Command {
    Command::new(assert_cmd::cargo::cargo_bin!("sv-mint"))
}

fn write_case(root: &std::path::Path, name: &str, sv: &str, rules_py: &str, toml: &str) -> (PathBuf, PathBuf) {
    let dir = root.join(name);
    let _ = create_dir_all(dir.join("plugins"));
    let svp = dir.join("input.sv");
    let tomlp = dir.join("sv-mint.toml");
    let rp = dir.join("plugins").join("rules.py");
    write(&svp, sv).unwrap();
    write(&tomlp, toml).unwrap();
    write(&rp, rules_py).unwrap();
    (tomlp, svp)
}

#[test]
fn ansi_ports_unused_should_be_detected() {
    let td = tempdir().unwrap();
    let root = td.path();
    let rules_py = r#"import sys, json
def main():
    for line in sys.stdin:
        m = json.loads(line)
        if m.get("type") != "CheckFileStage":
            continue
        st = m.get("stage")
        if st != "ast":
            print(json.dumps({"type":"ViolationsStage","stage":st,"violations":[]}))
            continue
        ast = m["payload"].get("ast", {})
        scopes = ast.get("symbol_table", {}).get("scopes", {})
        vs = []
        def emit(name,loc,rule_id):
            l = int(loc.get("line",1))
            c = int(loc.get("col",1))
            vs.append({"severity":"warning","rule_id":rule_id,"message":f"'{name}' declared but never used","location":{"line":l,"col":c,"end_line":l,"end_col":c}})
        for sc,idents in scopes.items():
            for ident,info in idents.items():
                klass = info.get("rw_class","unused")
                d = info.get("decl",{})
                dt = d.get("decl_type","Other")
                if klass == "unused":
                    if dt == "Param":
                        emit(ident,d,"decl.unused.param")
                    elif dt == "Typedef":
                        emit(ident,d,"decl.unused.typedef")
                    else:
                        emit(ident,d,"decl.unused")
        print(json.dumps({"type":"ViolationsStage","stage":st,"violations":vs}))
if __name__ == "__main__":
    main()
"#;
    let toml = r#"[defaults]
timeout_ms_per_file = 3000

[plugin]
cmd = "py"
args = ["-3","-u","plugins/rules.py"]

[stages]
enabled = ["pp_text","ast"]

[svparser]
include_paths = []
defines = []
strip_comments = true
ignore_include = false
allow_incomplete = true

[rules]
"#;

    let sv_in_unused = "module top(\n  input logic a\n);\nendmodule\n";
    let (cfg1, inp1) = write_case(root, "in_unused", sv_in_unused, rules_py, toml);
    let mut c1 = bin();
    c1.arg("--config").arg(&cfg1).arg(&inp1);
    c1.assert().code(2).stdout(
        predicate::str::contains("decl.unused")
            .and(predicate::str::contains(":2:15:"))
            .and(predicate::str::contains("a")),
    );

    let sv_out_unused = "module top(\n  output logic q\n);\nendmodule\n";
    let (cfg2, inp2) = write_case(root, "out_unused", sv_out_unused, rules_py, toml);
    let mut c2 = bin();
    c2.arg("--config").arg(&cfg2).arg(&inp2);
    c2.assert().code(2).stdout(
        predicate::str::contains("decl.unused")
            .and(predicate::str::contains(":2:16:"))
            .and(predicate::str::contains("q")),
    );
}
