# Rust SaaS Backend

A production-grade, modular **Rust backend starter** for building SaaS applications using **Axum, PostgreSQL, SeaORM, JWT authentication, and Stripe billing**.

This project is designed to:
- Serve as a **real-world SaaS backend template**
- Be **interview-ready** for Rust backend roles
- Scale cleanly to multi-tenant, subscription-based products
- Power real products like **100daysofchallenge.io**

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

## ⚙️ Getting Started

### 1️⃣ Prerequisites

* Rust 1.75+
* Docker
* PostgreSQL (or Docker)

---

### 2️⃣ Clone & Setup

```bash
git clone https://github.com/yourname/rust-saas-backend.git
cd rust-saas-backend
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
DATABASE_URL=postgres://postgres:postgres@localhost:5432/saas
JWT_SECRET=supersecretkey
SERVER_ADDR=0.0.0.0:3000
STRIPE_SECRET_KEY=sk_test_...
STRIPE_WEBHOOK_SECRET=whsec_...
```

---

## 🧪 Testing

```bash
cargo test
```

---

## 🐳 Docker

```bash
docker build -t rust-saas-backend .
docker run -p 3000:3000 rust-saas-backend
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
