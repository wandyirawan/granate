# Granate - Headless CMS in Rust

A headless content management system built with Rust, Axum, and PostgreSQL with JSONB support.

## Quick Start

### Prerequisites
- Rust (latest stable)
- Docker and Docker Compose
- sqlx-cli (optional, for manual migrations)

### Setup

1. Start PostgreSQL:
```bash
docker-compose up -d
```

2. Run the application:
```bash
cargo run
```

The server will start on `http://localhost:3000`.

### API Endpoints

- `GET /health` - Health check
- `POST /api/v1/auth/register` - Register a new user
- `POST /api/v1/auth/login` - Login
- `POST /api/v1/entries` - Create an entry
- `GET /api/v1/entries` - List entries
- `GET /api/v1/entries/:id` - Get entry by ID
- `PUT /api/v1/entries/:id` - Update entry
- `DELETE /api/v1/entries/:id` - Delete entry

### Environment Variables

Copy `.env.example` to `.env` and configure:

- `DATABASE_URL` - PostgreSQL connection string
- `JWT_SECRET` - Secret key for JWT tokens
- `PORT` - Server port (default: 3000)
