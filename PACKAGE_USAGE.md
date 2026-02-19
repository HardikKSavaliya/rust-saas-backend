# Using rust-saas-boilerplate as a Package

Complete guide on how to use `rust-saas-boilerplate` as a package dependency in your project.

**Now available on crates.io!** Use it just like `axum` - simple and clean.

## Quick Start

### From crates.io (Recommended) ⭐

```toml
[dependencies]
rust-saas-boilerplate = "0.1.0"  # Just like axum = "0.8"!
```

That's it! No paths, no git URLs - just like any other Rust crate.

### 1. Add to Your Project's `Cargo.toml`

**Option A: From crates.io (Recommended)**
```toml
[dependencies]
rust-saas-boilerplate = "0.1.0"
```

Just like using `axum = "0.8"`, you can now use `rust-saas-boilerplate = "0.1.0"`!

**Option B: Local Path (Development)**
```toml
[dependencies]
rust-saas-boilerplate = { path = "../rust-saas-boilerplate" }
```

**Option C: Git Repository**
```toml
[dependencies]
rust-saas-boilerplate = { git = "https://github.com/HardikKSavaliya/rust-saas-backend.git" }
```

**Option D: Specific Branch/Tag**
```toml
[dependencies]
rust-saas-boilerplate = { 
    git = "https://github.com/HardikKSavaliya/rust-saas-backend.git",
    branch = "main"
}
# Or
rust-saas-boilerplate = { 
    git = "https://github.com/HardikKSavaliya/rust-saas-backend.git",
    tag = "v0.1.0"
}
```

### 2. Use in Your Code

**Basic Usage - Full App:**
```rust
use rust_saas_boilerplate::create_app;

#[tokio::main]
async fn main() {
    let app = create_app();
    // Start server...
}
```

**Using Health Module Only:**
```rust
use axum::Router;
use rust_saas_boilerplate::modules::health;

#[tokio::main]
async fn main() {
    let health_routes = health::routes::health_routes();
    
    let app = Router::new()
        .merge(health_routes);
    
    // Start server...
}
```

## Complete Example Project

### Step 1: Create New Project

```bash
cargo new my-saas-app
cd my-saas-app
```

### Step 2: Update `Cargo.toml`

```toml
[package]
name = "my-saas-app"
version = "0.1.0"
edition = "2021"

[dependencies]
# Use from crates.io - just like axum!
rust-saas-boilerplate = "0.1.0"
axum = "0.8"
tokio = { version = "1.49", features = ["full"] }
tower = "0.5"
tower-http = "0.6"
```

### Step 3: Create `src/main.rs`

```rust
use axum::{Router, routing::get};
use rust_saas_boilerplate::modules::health;

async fn my_handler() -> &'static str {
    "Hello from my SaaS app!"
}

#[tokio::main]
async fn main() {
    // Get health routes from the package
    let health_routes = health::routes::health_routes();
    
    // Create your custom routes
    let my_routes = Router::new()
        .route("/api/hello", get(my_handler));
    
    // Combine everything
    let app = Router::new()
        .merge(health_routes)  // Health: /health, /, /example/*
        .merge(my_routes);      // Your routes: /api/hello
    
    // Start server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("🚀 Server running on http://localhost:3000");
    println!("   Health check: http://localhost:3000/health");
    println!("   Your endpoint: http://localhost:3000/api/hello");
    
    axum::serve(listener, app).await.unwrap();
}
```

### Step 4: Run

```bash
cargo run
```

## Available Exports

### Main Functions
- `create_app()` - Create the full application router
- `init_logging(config)` - Initialize logging based on environment
- `AppConfig` - Configuration struct
- `AppError` - Error type
- `AppResult<T>` - Result type alias

### Modules
- `modules::health` - Health check module
  - `health::routes::health_routes()` - Get health routes
  - `health::handler::health_check()` - Health check handler
  - `health::handler::root()` - Root handler

## Usage Patterns

### Pattern 1: Use Full Boilerplate

```rust
use rust_saas_boilerplate::create_app;

let app = create_app();
// Includes all routes from the boilerplate
```

### Pattern 2: Use Specific Modules

```rust
use rust_saas_boilerplate::modules::health;

let health_routes = health::routes::health_routes();
// Use only health check functionality
```

### Pattern 3: Mix and Match

```rust
use rust_saas_boilerplate::{create_app, modules::health, AppConfig, init_logging};

// Initialize logging
let config = AppConfig::from_env();
init_logging(&config);

// Get full app
let saas_app = create_app();

// Get additional health routes if needed
let extra_health = health::routes::health_routes();

// Combine
let app = Router::new()
    .merge(saas_app)
    .merge(extra_health);
```

### Pattern 4: Use Individual Components

```rust
use rust_saas_boilerplate::{AppConfig, AppError, AppResult};

// Use config
let config = AppConfig::from_env();

// Use error types
fn my_handler() -> AppResult<String> {
    Ok("Success".to_string())
}
```

## Testing the Package

After adding the dependency, verify it works:

```bash
# In your project directory
cargo check
```

If it compiles successfully, the package is properly integrated!

## Troubleshooting

### Issue: Can't find module

**Solution:** Make sure the path in `Cargo.toml` is correct relative to your project.

### Issue: Version conflicts

**Solution:** Ensure compatible versions of `axum`, `tokio`, etc. are used in both projects.

### Issue: Module not exported

**Solution:** All modules are exported via `src/lib.rs`. Use `rust_saas_boilerplate::modules::health`.

## Next Steps

- See [`HEALTH_USAGE.md`](./HEALTH_USAGE.md) for detailed health module usage
- See [`EXAMPLES.md`](./EXAMPLES.md) for more code examples
- See [`USAGE.md`](./USAGE.md) for general usage guide
