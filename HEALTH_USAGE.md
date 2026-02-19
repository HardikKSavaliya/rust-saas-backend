# Using Health Module in Other Repositories

This guide shows how to use the `health` module from `rust-saas-boilerplate` in your own project.

## Installation

Add the dependency to your `Cargo.toml`:

```toml
[dependencies]
# From crates.io - just like axum!
rust-saas-boilerplate = "0.1.0"
axum = "0.8"
tokio = { version = "1.49", features = ["full"] }
```

Or from other sources:
```toml
# From local path
rust-saas-boilerplate = { path = "../rust-saas-boilerplate" }

# From git
rust-saas-boilerplate = { git = "https://github.com/HardikKSavaliya/rust-saas-backend.git" }
```

## Usage Examples

### Example 1: Use Health Routes Only

```rust
use axum::Router;
use rust_saas_boilerplate::modules::health;

#[tokio::main]
async fn main() {
    // Get health check routes
    let health_routes = health::routes::health_routes();
    
    // Create your app with health routes
    let app = Router::new()
        .merge(health_routes);
    
    // Start server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
```

### Example 2: Combine Health Routes with Your Own Routes

```rust
use axum::{Router, routing::get};
use rust_saas_boilerplate::modules::health;

async fn my_custom_handler() -> &'static str {
    "My custom endpoint"
}

#[tokio::main]
async fn main() {
    // Get health routes from boilerplate
    let health_routes = health::routes::health_routes();
    
    // Create your custom routes
    let my_routes = Router::new()
        .route("/api/custom", get(my_custom_handler));
    
    // Combine them
    let app = Router::new()
        .merge(health_routes)  // Health routes at root: /health, /
        .nest("/api", my_routes);  // Your routes under /api
    
    // Start server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
```

### Example 3: Use Individual Health Handlers

```rust
use axum::{Router, routing::get};
use rust_saas_boilerplate::modules::health::handler;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/health", get(handler::health_check))
        .route("/", get(handler::root))
        // Add your own routes
        .route("/custom", get(|| async { "Custom" }));
    
    // Start server...
}
```

### Example 4: Use Health Routes with Custom Path Prefix

```rust
use axum::Router;
use rust_saas_boilerplate::modules::health;

#[tokio::main]
async fn main() {
    let health_routes = health::routes::health_routes();
    
    let app = Router::new()
        .nest("/monitoring", health_routes);  // Health routes under /monitoring
        // Now accessible at /monitoring/health, /monitoring/
    
    // Start server...
}
```

### Example 5: Use Health Module with Full Boilerplate App

```rust
use axum::Router;
use rust_saas_boilerplate::{create_app, modules::health};

#[tokio::main]
async fn main() {
    // Get the full boilerplate app
    let saas_app = create_app();
    
    // Get health routes separately (if you want to customize)
    let health_routes = health::routes::health_routes();
    
    // Combine or use separately
    let app = Router::new()
        .merge(saas_app)  // Full boilerplate routes
        .merge(health_routes);  // Additional health routes if needed
    
    // Start server...
}
```

## Available Health Endpoints

When you use `health::routes::health_routes()`, you get:

- `GET /health` - Health check endpoint (returns "OK")
- `GET /` - Root endpoint (returns "Rust SaaS Backend API")
- `GET /example/success` - Example success endpoint
- `GET /example/error` - Example error endpoint
- `GET /example/result` - Example result endpoint

## Custom Health Check

You can also create your own health check handler:

```rust
use axum::{Router, routing::get, Json};
use rust_saas_boilerplate::modules::health;
use serde_json::json;

async fn custom_health_check() -> Json<serde_json::Value> {
    Json(json!({
        "status": "healthy",
        "service": "my-service",
        "timestamp": chrono::Utc::now()
    }))
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/health", get(custom_health_check))
        .merge(health::routes::health_routes());
    
    // Start server...
}
```

## Module Structure

The health module exports:

- `health::routes::health_routes()` - Get all health routes as a Router
- `health::handler::health_check()` - Health check handler function
- `health::handler::root()` - Root handler function
- `health::handler::example_*()` - Example handlers for learning

## Full Example Project

Create a new project:

```bash
cargo new my-app
cd my-app
```

Add to `Cargo.toml`:

```toml
[package]
name = "my-app"
version = "0.1.0"
edition = "2021"

[dependencies]
# From crates.io - just like axum!
rust-saas-boilerplate = "0.1.0"
axum = "0.8"
tokio = { version = "1.49", features = ["full"] }
tower = "0.5"
tower-http = "0.6"
```

In `src/main.rs`:

```rust
use axum::{Router, routing::get};
use rust_saas_boilerplate::modules::health;

async fn hello() -> &'static str {
    "Hello from my app!"
}

#[tokio::main]
async fn main() {
    // Use health routes from boilerplate
    let health_routes = health::routes::health_routes();
    
    // Your custom routes
    let app = Router::new()
        .route("/hello", get(hello))
        .merge(health_routes);
    
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Server running on http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}
```

Now you can:
- Access `/health` for health checks
- Access `/` for root endpoint
- Access `/hello` for your custom endpoint
