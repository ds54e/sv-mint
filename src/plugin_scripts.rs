use crate::config::Config;
use std::path::Path;
use std::collections::{HashMap, BTreeSet};

pub fn resolve_script_path(s: &str) -> String {
    let p = Path::new(s);
    if p.is_absolute() && p.exists() {
        return p.to_string_lossy().into_owned();
    }
    if let Ok(cwd) = std::env::current_dir() {
        let c = cwd.join(s);
        if c.exists() {
            return c.to_string_lossy().into_owned();
        }
    }
    if let Ok(exe) = std::env::current_exe() {
        if let Some(base) = exe.parent().and_then(|d| d.parent()) {
            let c = base.join(s);
            if c.exists() {
                return c.to_string_lossy().into_owned();
            }
        }
    }
    s.to_string()
}

pub struct ScriptSpec {
    pub path: String,
    pub stages: Vec<String>,
}

pub fn collect_script_specs(cfg: &Config) -> Vec<ScriptSpec> {
    let mut order: Vec<String> = Vec::new();
    let mut stages: HashMap<String, BTreeSet<String>> = HashMap::new();
    for rule in &cfg.rule {
        let path = resolve_script_path(&rule.script);
        if !stages.contains_key(&path) {
            order.push(path.clone());
        }
        stages
            .entry(path)
            .or_insert_with(BTreeSet::new)
            .insert(rule.stage.as_str().to_string());
    }
    order
        .into_iter()
        .map(|path| {
            let stage_set = stages.remove(&path).unwrap_or_default();
            ScriptSpec {
                path,
                stages: stage_set.into_iter().collect(),
            }
        })
        .collect()
}
