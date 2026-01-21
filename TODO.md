# Rust SaaS Backend — Roadmap

This document tracks development progress for building a **production-grade SaaS backend** in Rust.

---

## ✅ Phase 1 — Core Platform (MVP)

### Project Setup
- [x] Project scaffolding
- [x] Axum server bootstrap
- [x] Structured logging (tracing)
- [ ] Environment config loading
- [ ] Error handling layer
- [x] Health check

### Auth & Users
- [ ] User registration
- [ ] User login
- [ ] Password hashing (Argon2)
- [ ] JWT access tokens
- [ ] JWT refresh tokens
- [ ] Email uniqueness enforcement
- [ ] Auth middleware
- [ ] `/users/me` endpoint

### Database
- [ ] PostgreSQL integration
- [ ] SQLx migrations
- [ ] User table schema
- [ ] Soft deletes
- [ ] DB health check

---

## 🚀 Phase 2 — SaaS Foundations

### Organizations & Multi-Tenancy
- [ ] Organization model
- [ ] Org-user membership table
- [ ] Role-based access control (owner/admin/member)
- [ ] Org-scoped JWT claims
- [ ] Middleware org guards

### Billing
- [ ] Stripe customer creation
- [ ] Subscription plans
- [ ] Checkout sessions
- [ ] Webhook verification
- [ ] Subscription lifecycle handling
- [ ] Feature gating by plan

---

## 🔐 Phase 3 — Security & Reliability

- [ ] Rate limiting
- [ ] Request ID middleware
- [ ] Input validation
- [ ] Email verification flow
- [ ] Password reset flow
- [ ] Token revocation
- [ ] Audit logging

---

## ⚙️ Phase 4 — Developer Experience

- [ ] OpenAPI / Swagger docs
- [ ] Postman collection
- [ ] Seed data scripts
- [ ] Integration test harness
- [ ] Testcontainers for Postgres
- [ ] CLI admin tool

---

## 📦 Phase 5 — Deployment & Ops

- [ ] Docker production build
- [ ] Fly.io deployment config
- [ ] Railway deployment config
- [ ] GitHub Actions CI
- [ ] Automated migrations on deploy
- [ ] Prometheus metrics
- [ ] Health + readiness probes

---

## 🌍 Phase 6 — SaaS Product Features

- [ ] API keys
- [ ] Usage quotas & rate plans
- [ ] Feature flags
- [ ] Admin dashboard endpoints
- [ ] Team invites
- [ ] Webhooks for customers
- [ ] Background job queue

---

## 🎯 Stretch Goals

- [ ] Multi-region deployment
- [ ] Distributed tracing
- [ ] Event sourcing
- [ ] CQRS read models
- [ ] Plugin system
- [ ] GraphQL gateway

---

## 🧠 Interview Readiness Checklist

- [ ] Explain Axum extractor model
- [ ] Explain SQLx compile-time queries
- [ ] Explain JWT auth vs sessions
- [ ] Explain multi-tenant schema design
- [ ] Explain Stripe webhook security
- [ ] Explain Rust async runtime model

---

## 🏁 Definition of Done (MVP)

- Auth + JWT working
- Org-based tenancy
- Stripe subscriptions
- Docker deployable
- CI passing
- README complete
