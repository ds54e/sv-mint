use super::RuleConfig;
use crate::errors::ConfigError;
use std::collections::HashSet;

pub fn apply_rule_overrides(rules: &mut [RuleConfig], only: &[String], disable: &[String]) -> Result<(), ConfigError> {
    if only.is_empty() && disable.is_empty() {
        return Ok(());
    }
    let existing: HashSet<String> = rules.iter().map(|r| r.id.clone()).collect();
    for id in only.iter().chain(disable.iter()) {
        if !existing.contains(id) {
            return Err(ConfigError::InvalidValue {
                detail: format!("rule {} not found", id),
            });
        }
    }
    if !only.is_empty() {
        let only_set: HashSet<&str> = only.iter().map(|s| s.as_str()).collect();
        for rule in rules.iter_mut() {
            rule.enabled = only_set.contains(rule.id.as_str());
        }
    }
    if !disable.is_empty() {
        let disable_set: HashSet<&str> = disable.iter().map(|s| s.as_str()).collect();
        for rule in rules.iter_mut() {
            if disable_set.contains(rule.id.as_str()) {
                rule.enabled = false;
            }
        }
    }
    Ok(())
}
