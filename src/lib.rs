//! Rust SaaS Boilerplate
//!
//! A production-grade Rust SaaS boilerplate with SeaORM + PostgreSQL integration.
//!
//! # Usage as Library
//!
//! ```rust,no_run
//! use rust_saas_boilerplate::{create_app, connect_database, AppConfig, AppState};
//!
//! # async {
//! let config = AppConfig::from_env();
//! let db = connect_database(&config.database_url).await.unwrap();
//! let state = AppState::new(db);
//! let app = create_app(state);
//! # };
//! ```
//!
//! # Usage as Binary
//!
//! ```bash
//! cargo run
//! ```

pub mod app;
pub mod config;
pub mod db;
pub mod error;
pub mod migration;
pub mod modules;
pub mod state;

pub use app::rust_saas;
pub use config::AppConfig;
pub use db::connect_database;
pub use error::{AppError, AppResult};
pub use state::AppState;

pub fn init_logging(config: &AppConfig) {
    use tracing_subscriber::{fmt, EnvFilter};

    let filter = if config.is_production() {
        EnvFilter::new("error")
    } else {
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"))
    };

    fmt().with_target(false).with_env_filter(filter).init();
}

pub fn create_app(state: AppState) -> axum::Router {
    app::rust_saas(state)
}
