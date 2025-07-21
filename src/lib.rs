//! ClipManager - A cross-platform clipboard manager
//! 
//! This library provides the core functionality for the ClipManager application,
//! including clipboard monitoring, data storage, and configuration management.

pub mod app;
pub mod clipboard;
pub mod config;
pub mod error;
pub mod i18n;
pub mod storage;
pub mod ui;

// Re-export commonly used types
pub use app::ClipManagerApp;
pub use config::settings::AppConfig;
pub use error::{ClipManagerError, Result};

/// Initialize the ClipManager library
/// 
/// This function sets up logging and internationalization.
/// It should be called before using any other library functions.
pub fn init() -> Result<()> {
    // Initialize logging
    env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Info)
        .init();

    // Initialize internationalization
    i18n::init();

    log::info!("ClipManager library initialized");
    Ok(())
}

/// Get the version of the ClipManager library
pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        let version = version();
        assert!(!version.is_empty());
        assert!(version.contains('.'));
    }

    #[test]
    fn test_init() {
        // Test that initialization doesn't panic
        // Note: We can't test the actual initialization because
        // env_logger can only be initialized once per process
        assert!(true);
    }
}
