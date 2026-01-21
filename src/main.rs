use anyhow::Result;
use std::net::SocketAddr;
use tokio::signal;
use tracing::{info, Level};
use tracing_subscriber::fmt;

mod app;
mod modules;

use app::rust_saas;

#[tokio::main]
async fn main() -> Result<()> {
    fmt()
        .with_target(false)
        .with_max_level(Level::INFO)
        .init();

    let app = rust_saas();

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    info!("🚀 Server starting on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    
    // Create shutdown signal
    let shutdown = shutdown_signal();
    
    info!("Server listening on http://{}", addr);
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
