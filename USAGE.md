# Using rust-saas-boilerplate as a Package

This guide explains how to use `rust-saas-boilerplate` as a dependency in your own Rust project.

## Installation

### Option 1: From crates.io (Recommended) ⭐

Just like using `axum`, you can now use it from crates.io:

```toml
[dependencies]
rust-saas-boilerplate = "0.1.0"
```

That's it! Simple and clean, just like any other Rust crate.

### Option 2: Local Path (Development)

If you have the repository locally:

```toml
[dependencies]
rust-saas-boilerplate = { path = "../rust-saas-boilerplate" }
```

### Option 3: Git Repository

If the repository is on GitHub or another git host:

```toml
[dependencies]
rust-saas-boilerplate = { git = "https://github.com/HardikKSavaliya/rust-saas-backend.git" }
```

To use a specific branch or tag:

```toml
[dependencies]
rust-saas-boilerplate = { git = "https://github.com/HardikKSavaliya/rust-saas-backend.git", branch = "main" }
# Or
rust-saas-boilerplate = { git = "https://github.com/HardikKSavaliya/rust-saas-backend.git", tag = "v0.1.0" }
```

## Basic Usage

### 1. Create an Axum Router

```rust
use rust_saas_boilerplate::create_app;
use axum::Router;

#[tokio::main]
async fn main() {
    let app = create_app();
    
    // You can merge with your own routes
    let combined_app = Router::new()
        .merge(app)
        .route("/custom", get(custom_handler));
    
    // Start server...
}
```

### 2. Use Individual Components

```rust
use rust_saas_boilerplate::{AppConfig, AppError, AppResult};

// Load configuration
let config = AppConfig::from_env();

// Use error types
fn my_handler() -> AppResult<String> {
    // Your code here
    Ok("Success".to_string())
}
```

### 3. Use Modules Directly

```rust
use rust_saas_boilerplate::modules::health;
use axum::Router;

// Use health check routes
let health_routes = health::routes::health_routes();

// Combine with your own routes
let app = Router::new()
    .merge(health_routes)  // Adds /health, /, /example/*
    .route("/custom", get(|| async { "Custom" }));

// Or use individual handlers
use rust_saas_boilerplate::modules::health::handler;
let app = Router::new()
    .route("/health", get(handler::health_check))
    .route("/", get(handler::root));
```

## Example: Integrating into Your Project

```rust
use axum::{Router, routing::get};
use rust_saas_boilerplate::create_app;

async fn my_custom_handler() -> &'static str {
    "My custom endpoint"
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get the rust-saas-boilerplate router
    let saas_app = create_app();
    
    // Create your own router
    let my_app = Router::new()
        .route("/my-endpoint", get(my_custom_handler));
    
    // Combine them
    let app = Router::new()
        .nest("/api", saas_app)
        .merge(my_app);
    
    // Start server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app).await?;
    
    Ok(())
}
```

## Available Exports

The library exports:

- `create_app()` - Main function to create the Axum router
- `AppConfig` - Configuration struct
- `AppError` - Error type for handlers
- `AppResult<T>` - Result type alias
- `modules` - All modules (health, users, etc.)

## Environment Variables

Make sure to set up your `.env` file with required variables:

```env
HOST=0.0.0.0
PORT=3000
DATABASE_URL=postgres://user:pass@localhost:5432/dbname
ENVIRONMENT=development
```

## Dependencies

When using this package, you'll need to ensure your `Cargo.toml` includes compatible versions of:

- `axum` (same version as used in rust-saas-boilerplate)
- `tokio` (same version)
- Other dependencies as needed

The package will handle its own dependencies, but you may need to align versions if you're using the same crates directly.
