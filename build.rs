use std::env;
use std::fs;
use std::path::Path;

fn main() {
    println!("cargo:rerun-if-changed=sv-mint.toml");
    let manifest = fs::read_to_string("sv-mint.toml").expect("failed to read sv-mint.toml");
    let value: toml::Value = manifest.parse().expect("invalid sv-mint.toml");
    let mut entries: Vec<(String, String)> = Vec::new();
    if let Some(rules) = value.get("rule").and_then(|v| v.as_array()) {
        for rule in rules {
            if let (Some(id), Some(script)) = (
                rule.get("id").and_then(|v| v.as_str()),
                rule.get("script").and_then(|v| v.as_str()),
            ) {
                entries.push((id.to_string(), script.to_string()));
            }
        }
    }
    entries.sort_by(|a, b| a.0.cmp(&b.0));
    let mut out = String::from("pub const DEFAULT_RULE_SCRIPTS: &[(&str, &str)] = &[\n");
    for (id, script) in entries {
        out.push_str(&format!("    ({:?}, {:?}),\n", id, script));
    }
    out.push_str("];\n");
    let out_path = Path::new(&env::var("OUT_DIR").unwrap()).join("default_rule_scripts.rs");
    fs::write(out_path, out).expect("failed to write default_rule_scripts.rs");
}
