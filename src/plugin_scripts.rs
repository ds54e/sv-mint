use crate::config::Config;
use crate::plugin::client::run_plugin_once_with_args;
use crate::types::Violation;
use anyhow::Result;
use serde_json::Value;
use std::path::Path;

fn resolve_script_path(s: &str) -> String {
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

pub fn run_plugins_for_stage(cfg: &Config, stage: &str, input_path: &Path, payload: Value) -> Result<Vec<Violation>> {
    let mut all = Vec::new();
    if cfg.ruleset.scripts.is_empty() {
        let v = run_plugin_once_with_args(cfg, &cfg.plugin.args, stage, input_path, payload)?;
        all.extend(v);
        return Ok(all);
    }
    for s in &cfg.ruleset.scripts {
        let sp = resolve_script_path(s);
        let mut args = cfg.plugin.args.clone();
        args.push(sp);
        let v = run_plugin_once_with_args(cfg, &args, stage, input_path, payload.clone())?;
        all.extend(v);
    }
    Ok(all)
}
