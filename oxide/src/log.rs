use env_logger::Env;

// Re-export log macros so you can use `oxide::log::info!` etc.
pub use log::{debug, error, info, trace, warn};

/// Initialize logging once (idempotent if called multiple times).
pub fn init() {
    // Default to INFO if RUST_LOG is not set
    let _ = env_logger::Builder::from_env(Env::default().default_filter_or("info"))
        .format_timestamp_secs()
        .try_init();
    info!("Logger initialized.");
}
