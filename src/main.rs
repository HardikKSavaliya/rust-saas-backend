use anyhow::Result;
use tokio::signal;
use tracing::{Level, info};
use tracing_subscriber::fmt;

mod app;
mod config;
mod error;
mod modules;

use app::rust_saas;
use config::AppConfig;

#[tokio::main]
async fn main() -> Result<()> {
    fmt().with_target(false).with_max_level(Level::INFO).init();

    // Load configuration from environment variables
    let config = AppConfig::from_env();
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
