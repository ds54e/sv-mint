use crate::config::Config;
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

pub fn resolve_scripts(cfg: &Config) -> Vec<String> {
    cfg.ruleset.scripts.iter().map(|s| resolve_script_path(s)).collect()
}
