# Rust SaaS Boilerplate

A production-grade, modular **Rust SaaS boilerplate** - A complete starter template for building SaaS applications using **Axum, PostgreSQL, SeaORM, JWT authentication, and Stripe billing**.

This boilerplate is designed to:
- Serve as a **complete SaaS application starter template**
- Be **interview-ready** for Rust backend roles
- Scale cleanly to multi-tenant, subscription-based products
- Power real products like **100daysofchallenge.io**
- Act as a **boilerplate** similar to ABP (ASP.NET Boilerplate) for Rust

---

## 🚀 Features

- ✅ REST API using Axum
- ✅ JWT-based authentication (access + refresh tokens)
- ✅ Secure password hashing (Argon2)
- ✅ PostgreSQL with SeaORM
- ✅ Modular domain-driven architecture
- ✅ Role-based access control (RBAC)
- ✅ Stripe subscriptions & webhooks (optional)
- ✅ Dockerized deployment
- ✅ Health checks & structured logging
- ✅ Ready for Fly.io / Railway / Render

---

## 🏗️ Architecture

```txt
src/
├── app.rs
├── main.rs
├── config/
├── db/
├── middleware/
├── modules/
│   ├── auth/
│   ├── users/
│   ├── billing/
│   ├── orgs/
│   └── health/
└── error.rs
```

Each module follows:

* `handler.rs` → HTTP layer
* `service.rs` → Business logic
* `model.rs` → DB/domain models
* `routes.rs` → Router wiring

---

## 🛠️ Tech Stack

| Layer      | Tech         |
| ---------- | ------------ |
| Language   | Rust         |
| Web        | Axum         |
| Runtime    | Tokio        |
| Database   | PostgreSQL   |
| ORM        | SeaORM       |
| Auth       | JWT + Argon2 |
| Billing    | Stripe       |
| Logging    | tracing      |
| Deployment | Docker       |

---

## 📦 Using as a Package/Boilerplate

This boilerplate can be used as a library dependency in other Rust projects or as a starting point for your SaaS application.

### Add to Your Project

```toml
[dependencies]
rust-saas-boilerplate = { path = "../rust-saas-boilerplate" }
# Or from git:
# rust-saas-boilerplate = { git = "https://github.com/HardikKSavaliya/rust-saas-backend.git" }
```

### Quick Start

```rust
use rust_saas_boilerplate::create_app;

let app = create_app();
// Use in your Axum router
```

See [`USAGE.md`](./USAGE.md) for detailed usage examples.

---

## ⚙️ Getting Started (Standalone Server)

### 1️⃣ Prerequisites

* Rust 1.75+
* Docker
* PostgreSQL (or Docker)

---

### 2️⃣ Clone & Setup

```bash
git clone https://github.com/HardikKSavaliya/rust-saas-backend.git
cd rust-saas-boilerplate
cp .env.example .env
```

---

### 3️⃣ Run Database

```bash
docker-compose up -d db
```

---

### 4️⃣ Run Migrations

```bash
# Using sea-orm-cli (install with: cargo install sea-orm-cli)
sea-orm-cli migrate up
```

---

### 5️⃣ Start Server

```bash
cargo run
```

Server runs at:

```
http://localhost:3000
```

---

## 🔐 Example API

```http
POST /auth/register
POST /auth/login
GET  /users/me
GET  /health
```

---

## 📦 Environment Variables

```env
# Server Configuration
HOST=0.0.0.0
PORT=3000
ENVIRONMENT=development  # or "production"

# Database
DATABASE_URL=postgres://postgres:postgres@localhost:5432/saas

# Authentication
JWT_SECRET=supersecretkey

# Billing (optional)
STRIPE_SECRET_KEY=sk_test_...
STRIPE_WEBHOOK_SECRET=whsec_...

# Logging (optional - overrides default)
# RUST_LOG=info,rust_saas_boilerplate=debug
```

### Logging Behavior

- **Production** (`ENVIRONMENT=production`): Only ERROR level logs are shown
- **Development** (`ENVIRONMENT=development`): All logs (INFO, DEBUG, etc.) are shown
- **Override**: Set `RUST_LOG` environment variable to override default behavior
- **Errors**: ERROR level logs always show in both environments

---

## 🧪 Testing

```bash
cargo test
```

---

## 🐳 Docker

```bash
docker build -t rust-saas-boilerplate .
docker run -p 3000:3000 rust-saas-boilerplate
```

---

## 📈 Roadmap

See [`TODO.md`](./TODO.md)

---

## 🧠 Why This Project

This repo demonstrates:

* Real-world Rust backend engineering
* Clean modular architecture
* Production SaaS patterns (auth, billing, tenancy)
* Scalable system design

Perfect for:

* Rust backend interviews
* SaaS MVPs
* Startup foundations

---

## 📜 License

MIT

---

## 🤝 Contributing

PRs welcome. Fork, branch, commit, and submit.

---

## ⭐ If this helped you

Give the repo a ⭐ and feel free to fork it for your own SaaS ideas!
