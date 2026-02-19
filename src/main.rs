use anyhow::Result;
use tokio::signal;
use tracing::info;
use tracing_subscriber::{fmt, EnvFilter};

mod app;
mod config;
mod db;
mod error;
mod migration;
mod modules;
mod state;

use app::rust_saas;
use config::AppConfig;
use state::AppState;

fn init_logging(config: &AppConfig) {
    let filter = if config.is_production() {
        EnvFilter::new("error")
    } else {
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"))
    };

    fmt().with_target(false).with_env_filter(filter).init();
}

#[tokio::main]
async fn main() -> Result<()> {
    let config = AppConfig::from_env();
    init_logging(&config);
    info!("database url: {}", config.database_url);
    info!("Connecting to database...");
    let db = db::connect_database(&config.database_url)
        .await
        .expect("Failed to connect to database");
    info!("Database connected successfully");

    let state = AppState::new(db);
    let addr = config.server_addr();
    let app = rust_saas(state);

    info!("Server starting on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    let shutdown = shutdown_signal();

    info!("Press Ctrl+C to shutdown gracefully");

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown)
        .await?;

    info!("Server shutdown complete");
    Ok(())
}

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
