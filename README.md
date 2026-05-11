# Granate

> *From the Spanish "granate" — pomegranate. Small fruit, packed with goodness.*

A **lightweight headless CMS** written in Rust. Designed to run comfortably on a **$5 VPS** (512MB RAM) while serving content at blistering speed. Think Strapi or Directus, but without the Node.js baggage — zero cold starts, single-digit millisecond latency, and a binary measured in megabytes, not hundreds.

## Philosophy

- **Minimal footprint** — low memory, tiny binary, fast startup
- **No bloat** — JSONB schema-less content stored in PostgreSQL, no ORM ceremony
- **Delegated auth** — trusts [Mangosteen](https://github.com/wandyirawan/mangosteen) (external IAM) via JWKS + RS256 JWT
- **API-first** — every feature is exposed as a REST endpoint, built for Jamstack and mobile backends
- **Batteries included, but swappable** — swap the DB, swap the auth provider, the core stays small

## Architecture

```
┌─────────────────────┐     JWT (RS256)     ┌─────────────────────┐
│    Mangosteen       │ ◄────────────────── │      Granate        │
│    (IAM Service)    │                     │    (CMS Service)    │
│                     │                     │                     │
│  DB: SQLite         │                     │  DB: PostgreSQL     │
│  - Users            │                     │  - Content Types    │
│  - Auth             │                     │  - Entries (JSONB)  │
│  - Roles            │                     │  - Tags             │
└─────────────────────┘                     └─────────────────────┘
```

- **No local auth** — Granate trusts Mangosteen's JWT tokens; user info comes from claims
- **No users table** — Granate is stateless regarding identity
- **Separate databases** — each service owns its data, no shared DB coupling
- **JWKS-based validation** — Granate fetches JWKS from Mangosteen's `/api/.well-known/jwks.json`

## Quick Start

### Prerequisites

- Rust 1.80+
- PostgreSQL 16+ (or `docker-compose up -d`)
- [Mangosteen](https://github.com/wandyirawan/mangosteen) running on port 4000 (provides auth)

### Setup

```bash
# 1. Clone and enter
git clone https://github.com/wandyirawan/granate && cd granate

# 2. Start PostgreSQL
docker-compose up -d

# 3. Configure environment
cp .env.example .env
# Edit .env and set:
# - DATABASE_URL=postgresql://granate:granate123@localhost:5432/granate
# - MANGOSTEEN_URL=http://localhost:4000
# - MANGOSTEEN_JWKS_URL=http://localhost:4000/api/.well-known/jwks.json

# 4. Run
cargo run
```

The server starts on `http://localhost:3000`.

### Environment Variables

| Variable | Description | Default |
|---|---|---|
| `DATABASE_URL` | PostgreSQL connection string | — |
| `MANGOSTEEN_URL` | Mangosteen base URL | `http://localhost:4000` |
| `MANGOSTEEN_JWKS_URL` | Mangosteen JWKS endpoint | `http://localhost:4000/api/.well-known/jwks.json` |
| `PORT` | HTTP listen port | `3000` |

## API

All endpoints under `/api/v1`. Public routes (no auth): `/auth/login`, `/auth/register`. Protected routes require JWT in `Authorization: Bearer <token>` header.

### Auth Endpoints

```bash
# Register new user (proxy to Mangosteen)
POST /api/v1/auth/register
Body: {"email": "user@example.com", "password": "password123", "name": "Optional"}

# Login (proxy to Mangosteen)
POST /api/v1/auth/login
Body: {"email": "user@example.com", "password": "password123"}
Response: {"access_token": "...", "refresh_token": "...", "expires_in": 3600}

# Get current user info (requires JWT)
GET /api/v1/auth/me
Header: Authorization: Bearer <access_token>
```

### Content Types

```bash
POST   /api/v1/content-types       Create
GET    /api/v1/content-types       List all
GET    /api/v1/content-types/{id}  Get by ID
PUT    /api/v1/content-types/{id}  Update
DELETE /api/v1/content-types/{id}  Delete
```

### Entries

```bash
POST   /api/v1/entries             Create (requires auth)
GET    /api/v1/entries             List all (public)
GET    /api/v1/entries/{id}        Get by ID (public)
PUT    /api/v1/entries/{id}        Update (requires auth)
DELETE /api/v1/entries/{id}        Delete (requires auth)
```

### Tags

```bash
POST   /api/v1/tags                Create (requires auth)
GET    /api/v1/tags                List all (public)
DELETE /api/v1/tags/{id}           Delete (requires auth)
```

### Health

```bash
GET /health
```

## Auth Flow

1. **Register/Login** → Granate proxies to Mangosteen, returns JWT tokens
2. **Subsequent requests** → Include `Authorization: Bearer <access_token>` header
3. **JWT Validation** → Granate fetches JWKS from Mangosteen, validates token, extracts claims (email, role)
4. **User Info** → `GET /api/v1/auth/me` proxies to Mangosteen's `/api/users/me`

## Why Rust?

- **Memory safe, no GC** — ownership model eliminates entire classes of bugs without a garbage collector
- **Blazing fast** — consistently ranked among the fastest web frameworks (Axum on Tokio)
- **Tiny binary** — single ~15MB static binary vs hundreds of MB for Node/Python equivalents
- **Low resource** — idles at ~8MB RAM, thrives where Node/Ruby/Django would OOM
- **Fearless concurrency** — async runtime handles thousands of connections on a single thread
- **Compile-time guarantees** — if it builds, it's free of data races, null pointers, and use-after-free

## Stack

| Layer | Choice |
|---|---|
| HTTP framework | [Axum](https://github.com/tokio-rs/axum) 0.8 |
| Async runtime | [Tokio](https://tokio.rs) |
| Database driver | [SQLx](https://github.com/launchbadge/sqlx) + PostgreSQL |
| Auth | JWKS + RS256 JWT via [jsonwebtoken](https://github.com/Keats/jsonwebtoken) |
| Serialization | [Serde](https://serde.rs) + JSONB |
| Observability | [Tracing](https://github.com/tokio-rs/tracing) |

## License

MIT
