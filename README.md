# Granate

> *From the Spanish "granate" — pomegranate. Small fruit, packed with goodness.*

A **lightweight headless CMS** written in Rust. Designed to run comfortably on a **$5 VPS** (512MB RAM) while serving content at blistering speed. Think Strapi or Directus, but without the Node.js baggage — zero cold starts, single-digit millisecond latency, and a binary measured in megabytes, not hundreds.

## Philosophy

- **Minimal footprint** — low memory, tiny binary, fast startup
- **No bloat** — JSONB schema-less content stored in PostgreSQL, no ORM ceremony
- **Delegated auth** — trusts [Mangosteen](https://github.com/anomalyco/mangosteen) (external IAM) via RS256 JWT
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

## Quick Start

### Prerequisites

- Rust 1.80+
- PostgreSQL 16+ (or `docker-compose up -d`)
- [Mangosteen](https://github.com/anomalyco/mangosteen) running (provides JWT tokens)

### Setup

```bash
# 1. Clone and enter
git clone https://github.com/anomalyco/granate && cd granate

# 2. Start PostgreSQL
docker-compose up -d

# 3. Configure environment
cp .env.example .env

# 4. Generate JWT key pair (share public key with Mangosteen)
openssl genpkey -algorithm RSA -out mangosteen_private_key.pem -pkeyopt rsa_keygen_bits:2048
openssl rsa -pubout -in mangosteen_private_key.pem -out mangosteen_public_key.pem

# 5. Run
cargo run --release
```

The server starts on `http://localhost:3000`.

### Environment Variables

| Variable | Description | Default |
|---|---|---|
| `DATABASE_URL` | PostgreSQL connection string | — |
| `MANGOSTEEN_JWT_PUBLIC_KEY_FILE` | Path to RSA public key PEM (from Mangosteen) | — |
| `PORT` | HTTP listen port | `3000` |

## API

All endpoints under `/api/v1` except health. JWT required for write operations.

### Content Types

```
POST   /api/v1/content-types       Create
GET    /api/v1/content-types       List all
GET    /api/v1/content-types/{id}  Get by ID
PUT    /api/v1/content-types/{id}  Update
DELETE /api/v1/content-types/{id}  Delete
```

### Entries

```
POST   /api/v1/entries             Create
GET    /api/v1/entries             List all
GET    /api/v1/entries/{id}        Get by ID
PUT    /api/v1/entries/{id}        Update
DELETE /api/v1/entries/{id}        Delete
```

### Tags

```
POST   /api/v1/tags                Create
GET    /api/v1/tags                List all
DELETE /api/v1/tags/{id}           Delete
```

### Health

```
GET /health
```

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
| Auth | RS256 JWT via [jsonwebtoken](https://github.com/Keats/jsonwebtoken) |
| Serialization | [Serde](https://serde.rs) + JSONB |
| Observability | [Tracing](https://github.com/tokio-rs/tracing) |

## License

MIT
