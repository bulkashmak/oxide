use env_logger::Env;

// Re-export macros from the `log` crate
pub use log::{debug, error, info, trace, warn};

/// Initialize the logger for Oxide.
pub fn init() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info"))
        .format_timestamp_secs()
        .init();
}
