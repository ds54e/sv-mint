use crate::config::Config;
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::path::Path;

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
        let path = resolve_script_path(&rule.script);
        let entry = specs.entry(path.clone()).or_insert_with(|| {
            order.push(path.clone());
            ScriptSpecBuilder::default()
        });
        entry.stages.insert(rule.stage.as_str().to_string());
        entry
            .stage_rules
            .entry(rule.stage.as_str().to_string())
            .or_insert_with(Vec::new)
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
