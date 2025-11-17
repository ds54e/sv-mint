use crate::config::Config;
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::path::{Path, PathBuf};

pub fn resolve_script_path(cfg: &Config, s: &str) -> String {
    let p = Path::new(s);
    if p.is_absolute() && p.exists() {
        return p.to_string_lossy().into_owned();
    }
    for candidate in iter_search_paths(cfg, s) {
        if candidate.exists() {
            return candidate.to_string_lossy().into_owned();
        }
    }
    s.to_string()
}

fn iter_search_paths(cfg: &Config, rel: &str) -> Vec<PathBuf> {
    let mut out = Vec::new();
    if let Some(root) = cfg.plugin.normalized_root.as_ref() {
        out.push(root.join(rel));
    }
    for extra in &cfg.plugin.normalized_search_paths {
        out.push(extra.join(rel));
    }
    if !out.is_empty() {
        return out;
    }
    if let Ok(cwd) = std::env::current_dir() {
        out.push(cwd.join(rel));
    }
    if let Ok(exe) = std::env::current_exe() {
        if let Some(base) = exe.parent() {
            out.push(base.join(rel));
        }
    }
    out
}

#[derive(Default)]
struct ScriptSpecBuilder {
    stages: BTreeSet<String>,
    stage_rules: BTreeMap<String, Vec<String>>,
}

pub struct ScriptSpec {
    pub path: String,
    pub stages: Vec<String>,
    pub stage_rules: BTreeMap<String, Vec<String>>,
}

pub fn collect_script_specs(cfg: &Config) -> Vec<ScriptSpec> {
    let mut order: Vec<String> = Vec::new();
    let mut specs: HashMap<String, ScriptSpecBuilder> = HashMap::new();
    for rule in &cfg.rule {
        let path = resolve_script_path(cfg, &rule.script);
        let stage = rule.stage();
        let entry = specs.entry(path.clone()).or_insert_with(|| {
            order.push(path.clone());
            ScriptSpecBuilder::default()
        });
        entry.stages.insert(stage.as_str().to_string());
        entry
            .stage_rules
            .entry(stage.as_str().to_string())
            .or_default()
            .push(rule.id.clone());
    }
    order
        .into_iter()
        .map(|path| {
            let builder = specs.remove(&path).unwrap_or_default();
            let mut stage_rules = BTreeMap::new();
            for (stage, mut ids) in builder.stage_rules {
                ids.sort();
                stage_rules.insert(stage, ids);
            }
            ScriptSpec {
                path,
                stages: builder.stages.into_iter().collect(),
                stage_rules,
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::io::config::load;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn resolves_script_from_search_paths_without_root() {
        let tmp = tempdir().unwrap();
        let search_dir = tmp.path().join("plugins-extra");
        fs::create_dir_all(&search_dir).unwrap();
        let script_path = search_dir.join("format.no_tabs.raw.py");
        fs::write(&script_path, "print('ok')").unwrap();
        let cfg_text = format!(
            r#"[plugin]
search_paths = ["{}"]

[[rule]]
id = "format.no_tabs"
script = "format.no_tabs.raw.py"
stage = "raw_text"
"#,
            search_dir.to_string_lossy()
        );
        let mut cfg = load(&cfg_text).unwrap();
        cfg.plugin.normalized_search_paths = vec![search_dir];
        let resolved = resolve_script_path(&cfg, &cfg.rule[0].script);
        assert_eq!(resolved, script_path.to_string_lossy().into_owned());
    }
}
