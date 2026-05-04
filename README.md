# Granate - Headless CMS in Rust

Headless CMS with flexible JSONB content storage, backed by PostgreSQL. Authentication is handled externally by [Mangosteen](https://github.com/wandyirawan/mangosteen).

## Architecture

- **Granate** (this repo): Content management, entries, tags, media
- **Mangosteen**: User management, JWT RS256 authentication

## Quick Start

```bash
# Start PostgreSQL
docker-compose up -d

# Set up Mangosteen JWT public key in .env
# Copy the public key from your Mangosteen instance

# Run migrations
cargo install sqlx-cli
sqlx migrate run

# Start the server
cargo run
```

## Environment Variables

| Variable | Description |
|----------|-------------|
| `DATABASE_URL` | PostgreSQL connection string |
| `MANGOSTEEN_JWT_PUBLIC_KEY` | RSA public key PEM from Mangosteen for JWT verification |
| `PORT` | Server port (default: 3000) |

## API Endpoints

### Public
- `GET /health` - Health check

### Protected (requires Mangosteen JWT in `Authorization: Bearer <token>`)
- `POST /api/v1/entries` - Create entry
- `GET /api/v1/entries` - List entries
- `GET /api/v1/entries/{id}` - Get entry
- `PUT /api/v1/entries/{id}` - Update entry
- `DELETE /api/v1/entries/{id}` - Delete entry
- `POST /api/v1/content-types` - Create content type
- `GET /api/v1/content-types` - List content types
- `GET /api/v1/content-types/{id}` - Get content type
- `PUT /api/v1/content-types/{id}` - Update content type
- `DELETE /api/v1/content-types/{id}` - Delete content type
- `POST /api/v1/tags` - Create tag
- `GET /api/v1/tags` - List tags
- `DELETE /api/v1/tags/{id}` - Delete tag
