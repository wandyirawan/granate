# Granate

> *From the Spanish "granate" — pomegranate. Small fruit, packed with goodness.*

A **lightweight headless CMS** written in Rust. Designed to run comfortably on a **$5 VPS** (512MB RAM) while serving content at blistering speed. Think Strapi or Directus, but without the Node.js baggage — zero cold starts, single-digit millisecond latency, and a binary measured in megabytes, not hundreds.

## Philosophy

- **Minimal footprint** — low memory, tiny binary, fast startup
- **No bloat** — JSONB schema-less content stored in PostgreSQL, no ORM ceremony
- **Delegated auth** — trusts [Mangosteen](https://github.com/wandyirawan/mangosteen) (external IAM) via JWKS + RS256 JWT
- **API-first** — every feature is exposed as a REST endpoint, built for Jamstack and mobile backends
- **Media management** — image upload with auto-generated variants (thumb, catalog, full), stored in Minio (S3-compatible)
- **Batteries included, but swappable** — swap the DB, swap the auth provider, the core stays small

## Architecture

```
┌─────────────────────┐     JWT (RS256)     ┌──────────────────────────┐
│    Mangosteen       │ ◄────────────────── │        Granate           │
│    (IAM Service)    │                     │      (CMS Service)       │
│                     │                     │                          │
│  DB: SQLite         │                     │  DB: PostgreSQL          │
│  - Users            │                     │  - Content Types         │
│  - Auth             │                     │  - Entries (JSONB)       │
│  - Roles            │                     │  - Tags                  │
└─────────────────────┘                     │  - Media + Variants      │
                                            │  - Product Pages         │
                                            │                          │
                                            │  Minio (S3)              │
                                            │  - granate-media bucket  │
                                            └──────────────────────────┘
```

- **No local auth** — Granate trusts Mangosteen's JWT tokens; user info comes from claims
- **No users table** — Granate is stateless regarding identity
- **Separate databases** — each service owns its data, no shared DB coupling
- **JWKS-based validation** — Granate fetches JWKS from Mangosteen's `/api/.well-known/jwks.json`

## Quick Start

### Prerequisites

- Rust 1.80+
- PostgreSQL 16+ (or `docker-compose up -d`)
- Minio (S3-compatible storage for media)
- [Mangosteen](https://github.com/wandyirawan/mangosteen) running on port 4000 (provides auth)

### Setup

```bash
# 1. Clone and enter
git clone https://github.com/wandyirawan/granate && cd granate

# 2. Start PostgreSQL + Minio (from central infra)
# See: https://github.com/wandyirawan/saladbuah (infra/)

# 3. Configure environment
cp .env.example .env
# Edit .env and set:
# - DATABASE_URL=postgresql://granate:granate123@localhost:5433/granate
# - MANGOSTEEN_URL=http://localhost:4000
# - MANGOSTEEN_JWKS_URL=http://localhost:4000/api/.well-known/jwks.json
# - MINIO_ENDPOINT=localhost:9000
# - MINIO_BUCKET=granate-media

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
| `MINIO_ENDPOINT` | Minio S3 endpoint | `localhost:9000` |
| `MINIO_ACCESS_KEY` | Minio access key | `pomegranate` |
| `MINIO_SECRET_KEY` | Minio secret key | `pomegranate123` |
| `MINIO_BUCKET` | Minio bucket name | `granate-media` |
| `PORT` | HTTP listen port | `3000` |

## API

All endpoints under `/api/v1`. Public routes (no auth): `/auth/login`, `/auth/register`, `/health`. Protected routes require JWT in `Authorization: Bearer <token>` header.

### Auth Endpoints

```bash
# Register new user (proxy to Mangosteen)
POST /api/v1/auth/register
Body: {"email": "user@example.com", "password": "***", "name": "Optional"}

# Login (proxy to Mangosteen)
POST /api/v1/auth/login
Body: {"email": "user@example.com", "password": "***"}
Response: {"access_token": "***", "refresh_token": "***", "expires_in": 3600}

# Get current user info (requires JWT)
GET /api/v1/auth/me
Header: Authorization: Bearer <token>
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
POST   /api/v1/entries             Create
GET    /api/v1/entries             List all
GET    /api/v1/entries/{id}        Get by ID
PUT    /api/v1/entries/{id}        Update
DELETE /api/v1/entries/{id}        Delete
```

### Tags

```bash
POST   /api/v1/tags                Create
GET    /api/v1/tags                List all
DELETE /api/v1/tags/{id}           Delete
```

### Media

```bash
POST /api/v1/media/upload          Upload image (multipart/form-data)
GET  /api/v1/media                 List media (latest 50)
```

Image upload automatically generates 3 variants:
- **thumb** — 200×200 center crop
- **catalog** — max 600×600, maintains aspect ratio
- **full** — max 1200×1200

All stored in Minio (`granate-media` bucket) under `media/{prefix}/{uuid}/` path.

```bash
# Upload
curl -X POST http://localhost:3000/api/v1/media/upload \
  -H "Authorization: Bearer <token>" \
  -F "file=@photo.jpg"

# Response includes original + variant URLs
```

### Health

```bash
GET /health
```

## Database

### Tables

| Table | Purpose |
|---|---|
| `content_types` | Content type definitions (schema) |
| `entries` | Content entries (JSONB data) |
| `tags` | Taxonomy tags |
| `media` | Uploaded images (filename, storage_key, dimensions) |
| `media_variants` | Image variants (thumb, catalog, full) per media |
| `product_pages` | CMS-rich product pages linked to Salak products |
| `product_blog_links` | Blog-to-product cross-references |

## Auth Flow

1. **Register/Login** → Granate proxies to Mangosteen, returns JWT tokens
2. **Subsequent requests** → Include `Authorization: Bearer <token>` header
3. **JWT Validation** → Granate fetches JWKS from Mangosteen, validates token, extracts claims (email, role)
4. **User Info** → `GET /api/v1/auth/me` proxies to Mangosteen's `/api/users/me`

## Why Rust?

- **Memory safe, no GC** — ownership model eliminates entire classes of bugs without a garbage collector
- **Blazing fast** — consistently ranked among the fastest web frameworks (Axum on Tokio)
- **Tiny binary** — single ~15MB static binary vs hundreds of MB for Node/Python equivalents
- **Low resource** — idles at ~8MB RAM, thrives where Node/Ruby/Django would OOM
- **Fearless concurrency** — async runtime handles thousands of connections on a single thread
- **Compile-time guarantees** — if it builds, it's free of data races, null pointers, and use-after-free

## Pomegranate Full Stack

Granate is the **body/CMS** of **Pomegranate** (🍎), combined with **Pome** (head frontend):

- **Pome** (Head) → Bun + Elysia + HTMX + Alpine.js + Pico CSS
- **Granate** (Body) → Rust + Axum + PostgreSQL (this repo)
- **Mangosteen** (Auth) → Go + Fiber + SQLite
- **Salak** (Product) → Python + FastAPI + Granian
- **Kelapa** (Ecommerce) → Elixir + Phoenix + Elm

See: https://github.com/wandyirawan/saladbuah

## Stack

| Layer | Choice |
|---|---|
| HTTP framework | [Axum](https://github.com/tokio-rs/axum) 0.8 |
| Async runtime | [Tokio](https://tokio.rs) |
| Database driver | [SQLx](https://github.com/launchbadge/sqlx) + PostgreSQL |
| Object storage | [aws-sdk-s3](https://github.com/awslabs/aws-sdk-rust) + Minio |
| Image processing | [image](https://github.com/image-rs/image) 0.25 |
| Auth | JWKS + RS256 JWT via [jsonwebtoken](https://github.com/Keats/jsonwebtoken) |
| Serialization | [Serde](https://serde.rs) + JSONB |
| Observability | [Tracing](https://github.com/tokio-rs/tracing) |

## License

MIT
