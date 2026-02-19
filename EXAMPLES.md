# Usage Examples

## Example 1: Basic Integration

Create a new project and add to `Cargo.toml`:

```toml
[package]
name = "my-app"
version = "0.1.0"
edition = "2021"

[dependencies]
# Use from crates.io - just like axum!
rust-saas-boilerplate = "0.1.0"
tokio = { version = "1.49", features = ["full"] }
axum = "0.8"
```

Then in your `src/main.rs`:

```rust
use axum::{Router, routing::get};
use rust_saas_boilerplate::create_app;

async fn my_handler() -> &'static str {
    "Hello from my app!"
}

#[tokio::main]
async fn main() {
    // Get the rust-saas-boilerplate router
    let saas_router = create_app();
    
    // Create your custom routes
    let my_router = Router::new()
        .route("/", get(my_handler));
    
    // Combine them
    let app = Router::new()
        .nest("/api", saas_router)  // rust-saas-boilerplate routes under /api
        .merge(my_router);           // Your routes at root
    
    // Start server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
```

## Example 2: Using Individual Components

```rust
use rust_saas_boilerplate::{AppConfig, AppError, AppResult};
use axum::{Router, routing::get, Json};
use serde_json::json;

async fn my_endpoint() -> AppResult<Json<serde_json::Value>> {
    let config = AppConfig::from_env();
    Ok(Json(json!({
        "message": "Using rust-saas-boilerplate components",
        "port": config.port
    })))
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/my-endpoint", get(my_endpoint));
    
    // Start server...
}
```

## Example 3: Using from crates.io (Recommended)

In your project's `Cargo.toml`:

```toml
[dependencies]
rust-saas-boilerplate = "0.1.0"  # Just like axum = "0.8"!
```

That's it! Simple and clean.

## Example 3b: Using from Git Repository

If you need the latest from git:

```toml
[dependencies]
rust-saas-boilerplate = { git = "https://github.com/HardikKSavaliya/rust-saas-backend.git", branch = "main" }
```

Or use a specific tag/version:

```toml
[dependencies]
rust-saas-boilerplate = { git = "https://github.com/HardikKSavaliya/rust-saas-backend.git", tag = "v0.1.0" }
```

## Example 4: Using Health Module

```rust
use rust_saas_boilerplate::modules::health;
use axum::Router;

fn create_my_app() -> Router {
    Router::new()
        .merge(health::routes::health_routes())
        // Health routes provide: /health, /, /example/*
        // Add your own routes
}

// Or use individual handlers:
use rust_saas_boilerplate::modules::health::handler;

fn create_my_app_with_custom_health() -> Router {
    Router::new()
        .route("/health", axum::routing::get(handler::health_check))
        .route("/", axum::routing::get(handler::root))
        // Add your own routes
}
```

## Example 5: Error Handling

```rust
use rust_saas_boilerplate::{AppError, AppResult};
use axum::{routing::get, Router};

async fn my_handler() -> AppResult<&'static str> {
    // Use AppError types
    if some_condition {
        Err(AppError::BadRequest("Invalid input".to_string()))
    } else {
        Ok("Success")
    }
}

fn routes() -> Router {
    Router::new()
        .route("/my-route", get(my_handler))
}
```
