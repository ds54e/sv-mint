mod input;
mod loader;
mod normalize;
mod overrides;
mod paths;
mod types;
mod validate;

pub use input::{read_input, InputText};
pub use loader::{load, load_from_path, resolve_path};
pub use overrides::apply_rule_overrides;
pub use paths::plugin_search_paths;
pub use types::{
    Config, Defaults, LogFormat, LoggingConfig, Plugin, RuleConfig, Stages, TransportConfig, TransportOnExceed,
};

#[cfg(test)]
mod tests;
