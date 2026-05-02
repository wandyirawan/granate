# Granate - Headless CMS in Rust

A headless content management system built with Rust, Axum, and PostgreSQL with JSONB support.
Uses **Mangosteen** as external IAM for authentication.

## Architecture

```
┌─────────────────────┐     JWT (RS256)     ┌─────────────────────┐
│    Mangosteen       │ ◄────────────────── │      Granate        │
│    (IAM Service)    │                     │    (CMS Service)    │
│                     │                     │                     │
│  DB: SQLite         │                     │  DB: PostgreSQL     │
│  - Users            │                     │  - Content Types    │
│  - Auth             │                     │  - Entries (JSONB)  │
│  - Roles            │                     │  - Media            │
└─────────────────────┘                     └─────────────────────┘
```

- **No local auth** - Granate trusts Mangosteen's JWT tokens
- **No users table** in Granate - author info comes from JWT claims
- **Each service has its own database** - no shared DB

## Quick Start

### Prerequisites
- Rust (latest stable)
- Go 1.25+ (for Mangosteen)
- Docker and Docker Compose

### Setup

1. Start PostgreSQL:
```bash
docker-compose up -d
```

2. Start Mangosteen (IAM):
```bash
cd ../mangosteen
cp .env.example .env
./generate-certs.sh
go run cmd/server/main.go
```

3. Run Granate:
```bash
cargo run
```

The server will start on `http://localhost:3000`.

### API Endpoints

- `GET /health` - Health check
- `POST /api/v1/entries` - Create an entry (requires JWT)
- `GET /api/v1/entries` - List entries
- `GET /api/v1/entries/:id` - Get entry by ID
- `PUT /api/v1/entries/:id` - Update entry (requires JWT)
- `DELETE /api/v1/entries/:id` - Delete entry (requires JWT)

### Environment Variables

Copy `.env.example` to `.env` and configure:

- `DATABASE_URL` - PostgreSQL connection string
- `MANGOSTEEN_JWKS_URL` - Mangosteen JWKS endpoint (default: http://localhost:8080/.well-known/jwks.json)
- `PORT` - Server port (default: 3000)
