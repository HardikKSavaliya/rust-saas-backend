//! Rust SaaS Boilerplate
//!
//! A production-grade Rust SaaS boilerplate - A complete starter template for building
//! SaaS applications. Can be used as a library dependency or run as a standalone server.
//!
//! This boilerplate provides:
//! - Complete authentication system
//! - User management
//! - Modular architecture
//! - Error handling
//! - Environment-based configuration
//! - And much more!
//!
//! # Usage as Library
//!
//! Add to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! rust-saas-boilerplate = "0.1.0"  # From crates.io - just like axum!
//! # Or from path:
//! # rust-saas-boilerplate = { path = "../rust-saas-boilerplate" }
//! # Or from git:
//! # rust-saas-boilerplate = { git = "https://github.com/HardikKSavaliya/rust-saas-backend.git" }
//! ```
//!
//! Then use it in your code:
//!
//! ```rust,no_run
//! use rust_saas_boilerplate::{create_app, AppConfig};
//!
//! let app = create_app();
//! // Use in your Axum router
//! ```
//!
//! # Usage as Binary
//!
//! ```bash
//! cargo run
//! ```

pub mod app;
pub mod config;
pub mod error;
pub mod modules;

// Re-export commonly used types
pub use app::rust_saas;
pub use config::AppConfig;
pub use error::{AppError, AppResult};

/// Initialize logging based on environment
/// - Production: Only ERROR level logs
/// - Development: All logs (INFO, DEBUG, etc.)
/// - Can be overridden by RUST_LOG environment variable
/// 
/// # Example
/// 
/// ```rust,no_run
/// use rust_saas_boilerplate::{init_logging, AppConfig};
/// 
/// let config = AppConfig::from_env();
/// init_logging(&config);
/// ```
pub fn init_logging(config: &AppConfig) {
    use tracing_subscriber::{fmt, EnvFilter};
    
    let filter = if config.is_production() {
        // Production: Only show ERROR level and above
        EnvFilter::new("error")
    } else {
        // Development: Show all logs (can be overridden by RUST_LOG env var)
        EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| EnvFilter::new("info"))
    };

    fmt()
        .with_target(false)
        .with_env_filter(filter)
        .init();
}

/// Create the application router
/// This is the main entry point for using this library
/// 
/// # Example
/// 
/// ```rust,no_run
/// use rust_saas_boilerplate::create_app;
/// use axum::Router;
/// 
/// let app: Router = create_app();
/// // Merge with your own routes or use as-is
/// ```
pub fn create_app() -> axum::Router {
    app::rust_saas()
}
