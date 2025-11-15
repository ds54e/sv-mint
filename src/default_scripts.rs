include!(concat!(env!("OUT_DIR"), "/default_rule_scripts.rs"));

pub fn lookup(rule_id: &str) -> Option<&'static str> {
    DEFAULT_RULE_SCRIPTS
        .iter()
        .find_map(|(id, script)| if *id == rule_id { Some(*script) } else { None })
}
