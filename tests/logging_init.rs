use sv_mint::config::LoggingConfig;
use sv_mint::diag::logging::init;

#[test]
fn logging_init_ok() {
    let cfg = LoggingConfig::default();
    let r = init(&cfg);
    assert!(r.is_ok());
}
