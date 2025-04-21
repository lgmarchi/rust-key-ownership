use base64::{
    alphabet,
    engine::{self, general_purpose},
};

pub mod crypto;
pub mod error;
pub mod types;

pub const BASE64_ENGINE: engine::GeneralPurpose =
    engine::GeneralPurpose::new(&alphabet::URL_SAFE, general_purpose::NO_PAD);

pub fn init_tracing() {
    use tracing_subscriber::fmt::format::FmtSpan;

    let log_env_filter = std::env::var("LOG_LEVEL").unwrap_or_else(|_| "info".into());

    tracing_subscriber::fmt()
        .with_env_filter(log_env_filter)
        .with_target(true)
        .with_level(true)
        .with_span_events(FmtSpan::CLOSE)
        .pretty()
        .init();
}
