use anyhow::Result;
use tokio::signal;
use tracing::info;
use tracing_subscriber::{fmt, EnvFilter};

mod app;
mod config;
mod error;
mod modules;

use app::rust_saas;
use config::AppConfig;

/// Initialize logging based on environment
/// - Production: Only ERROR level logs
/// - Development: All logs (INFO, DEBUG, etc.)
fn init_logging(config: &AppConfig) {
    let filter = if config.is_production() {
        // Production: Only show ERROR level and above
        EnvFilter::new("error")
    } else {
        // Development: Show all logs (can be overridden by RUST_LOG env var)
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"))
    };

    fmt().with_target(false).with_env_filter(filter).init();
}

#[tokio::main]
async fn main() -> Result<()> {
    // Load configuration from environment variables first
    let config = AppConfig::from_env();

    // Initialize logging based on environment
    init_logging(&config);

    let addr = config.server_addr();

    let app = rust_saas();

    info!("🚀 Server starting on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;

    // Create shutdown signal
    let shutdown = shutdown_signal();

    info!("Press Ctrl+C to shutdown gracefully");

    // Start server with graceful shutdown
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown)
        .await?;

    info!("Server shutdown complete");
    Ok(())
}

/// Handles shutdown signals (SIGTERM, SIGINT)
async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {
            info!("Received Ctrl+C, starting graceful shutdown...");
        },
        _ = terminate => {
            info!("Received SIGTERM, starting graceful shutdown...");
        },
    }
}
