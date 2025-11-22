use super::Config;
use std::collections::HashSet;
use std::env;
use std::path::PathBuf;

fn push_unique(out: &mut Vec<PathBuf>, seen: &mut HashSet<PathBuf>, p: PathBuf) {
    if seen.insert(p.clone()) {
        out.push(p);
    }
}

pub fn plugin_search_paths(cfg: &Config, rel: &str) -> Vec<PathBuf> {
    let mut out = Vec::new();
    let mut seen = HashSet::new();
    let has_user_roots = cfg.plugin.normalized_root.is_some() || !cfg.plugin.normalized_search_paths.is_empty();
    if let Some(root) = cfg.plugin.normalized_root.as_ref() {
        push_unique(&mut out, &mut seen, root.join(rel));
    }
    for extra in &cfg.plugin.normalized_search_paths {
        push_unique(&mut out, &mut seen, extra.join(rel));
    }
    if !has_user_roots && cfg.plugin.config_dir.is_some() {
        let config_dir = cfg.plugin.config_dir.as_ref().unwrap();
        push_unique(&mut out, &mut seen, config_dir.join("plugins").join(rel));
    }
    if !out.is_empty() {
        return out;
    }
    if let Ok(cwd) = env::current_dir() {
        push_unique(&mut out, &mut seen, cwd.join(rel));
    }
    if let Ok(exe) = env::current_exe() {
        if let Some(base) = exe.parent() {
            push_unique(&mut out, &mut seen, base.join(rel));
        }
    }
    out
}
