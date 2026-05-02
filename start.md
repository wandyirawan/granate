```text
# GRANATE - Headless CMS in Rust (Hybrid PostgreSQL+JSONB style)
# Aider bootstrap file - sequential implementation guide

## Phase 1: Project setup (complete first)

```bash
cargo new granate
cd granate
```

Update `Cargo.toml`:

```toml
[package]
name = "granate"
version = "0.1.0"
edition = "2021"

[dependencies]
axum = "0.8"
tokio = { version = "1", features = ["full"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
sqlx = { version = "0.8", features = ["runtime-tokio-native-tls", "postgres", "chrono", "uuid", "json"] }
uuid = { version = "1", features = ["v4", "serde"] }
chrono = { version = "0.4", features = ["serde"] }
tower = "0.4"
tower-http = { version = "0.5", features = ["cors", "trace"] }
anyhow = "1"
thiserror = "1"
dotenvy = "0.15"
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }
argon2 = "0.5"
jsonwebtoken = "9"
validator = { version = "0.18", features = ["derive"] }
async-trait = "0.1"

[dev-dependencies]
tokio-test = "0.4"
```

Create `.env`:

```env
DATABASE_URL=postgresql://granate:granate123@localhost:5432/granate
JWT_SECRET=change_this_to_random_string_in_production
PORT=3000
```

Create `docker-compose.yml`:

```yaml
version: '3.8'

services:
  postgres:
    image: postgres:16-alpine
    environment:
      POSTGRES_USER: granate
      POSTGRES_PASSWORD: granate123
      POSTGRES_DB: granate
    ports:
      - "5432:5432"
    volumes:
      - postgres_data:/var/lib/postgresql/data
    healthcheck:
      test: ["CMD-SHELL", "pg_isready -U granate"]
      interval: 5s
      timeout: 5s
      retries: 5

volumes:
  postgres_data:
```

## Phase 2: Database schema (migrations)

Create `migrations/20260101000000_initial_schema.sql`:

```sql
-- Enable UUID extension
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Users table (relational, strict)
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    email TEXT UNIQUE NOT NULL,
    password_hash TEXT NOT NULL,
    name TEXT NOT NULL,
    role TEXT NOT NULL DEFAULT 'editor', -- admin, editor, viewer
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Content types schema (flexible storage)
CREATE TABLE content_types (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name TEXT UNIQUE NOT NULL, -- e.g., "blog_post", "product"
    slug TEXT UNIQUE NOT NULL,
    description TEXT,
    schema_json JSONB NOT NULL DEFAULT '{}', -- JSON Schema for validation
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Content entries (hybrid: IDs + JSONB)
CREATE TABLE entries (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    content_type_id UUID NOT NULL REFERENCES content_types(id) ON DELETE CASCADE,
    author_id UUID NOT NULL REFERENCES users(id),
    slug TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'draft', -- draft, published, archived
    data JSONB NOT NULL DEFAULT '{}', -- Main content fields (flexible)
    meta JSONB NOT NULL DEFAULT '{}',  -- SEO, custom meta fields
    published_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(content_type_id, slug) -- Unique slug per content type
);

-- Tags (relational many-to-many via junction)
CREATE TABLE tags (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name TEXT UNIQUE NOT NULL,
    slug TEXT UNIQUE NOT NULL
);

CREATE TABLE entry_tags (
    entry_id UUID REFERENCES entries(id) ON DELETE CASCADE,
    tag_id UUID REFERENCES tags(id) ON DELETE CASCADE,
    PRIMARY KEY (entry_id, tag_id)
);

-- Media assets
CREATE TABLE media (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    filename TEXT NOT NULL,
    key TEXT UNIQUE NOT NULL, -- S3 key or local path
    mime_type TEXT NOT NULL,
    size_bytes BIGINT NOT NULL,
    width INT,
    height INT,
    uploaded_by UUID NOT NULL REFERENCES users(id),
    metadata JSONB DEFAULT '{}',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Indexes for performance
CREATE INDEX idx_entries_content_type ON entries(content_type_id);
CREATE INDEX idx_entries_author ON entries(author_id);
CREATE INDEX idx_entries_status ON entries(status);
CREATE INDEX idx_entries_published_at ON entries(published_at);
CREATE INDEX idx_entries_data_gin ON entries USING GIN(data); -- JSONB search
CREATE INDEX idx_users_email ON users(email);
CREATE INDEX idx_tags_slug ON tags(slug);

-- Automatic updated_at trigger
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ language 'plpgsql';

CREATE TRIGGER update_users_updated_at BEFORE UPDATE ON users
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_content_types_updated_at BEFORE UPDATE ON content_types
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_entries_updated_at BEFORE UPDATE ON entries
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
```

## Phase 3: Core Rust structure

Create `src/main.rs`:

```rust
mod config;
mod db;
mod models;
mod api;
mod auth;
mod error;

use axum::{Router, routing::get};
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;
use std::net::SocketAddr;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "granate=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    dotenvy::dotenv().ok();
    let config = config::Config::from_env()?;
    
    let pool = db::create_pool(&config.database_url).await?;
    sqlx::migrate!().run(&pool).await?;
    
    let app = Router::new()
        .route("/health", get(api::health::handler))
        .nest("/api/v1", api::router(pool))
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http());
    
    let addr = SocketAddr::from(([0, 0, 0, 0], config.port));
    tracing::info!("Granate CMS starting on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;
    
    Ok(())
}
```

Create `src/config.rs`:

```rust
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub database_url: String,
    pub jwt_secret: String,
    pub port: u16,
}

impl Config {
    pub fn from_env() -> anyhow::Result<Self> {
        Ok(Config {
            database_url: std::env::var("DATABASE_URL")?,
            jwt_secret: std::env::var("JWT_SECRET")?,
            port: std::env::var("PORT")
                .unwrap_or_else(|_| "3000".to_string())
                .parse()?,
        })
    }
}
```

Create `src/db.rs`:

```rust
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::time::Duration;

pub async fn create_pool(database_url: &str) -> anyhow::Result<PgPool> {
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .acquire_timeout(Duration::from_secs(5))
        .connect(database_url)
        .await?;
    
    Ok(pool)
}
```

Create `src/error.rs`:

```rust
use axum::{http::StatusCode, response::{IntoResponse, Response}, Json};
use serde_json::json;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    
    #[error("Not found")]
    NotFound,
    
    #[error("Validation error: {0}")]
    Validation(String),
    
    #[error("Unauthorized")]
    Unauthorized,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::Database(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Database error"),
            AppError::NotFound => (StatusCode::NOT_FOUND, "Not found"),
            AppError::Validation(msg) => (StatusCode::BAD_REQUEST, &msg),
            AppError::Unauthorized => (StatusCode::UNAUTHORIZED, "Unauthorized"),
        };
        
        (status, Json(json!({ "error": error_message }))).into_response()
    }
}
```

Create `src/models.rs`:

```rust
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use sqlx::FromRow;
use serde_json::Value as JsonValue;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub name: String,
    pub role: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct ContentType {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub schema_json: JsonValue,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Entry {
    pub id: Uuid,
    pub content_type_id: Uuid,
    pub author_id: Uuid,
    pub slug: String,
    pub status: String,
    pub data: JsonValue,
    pub meta: JsonValue,
    pub published_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
```

Create `src/api.rs`:

```rust
use axum::{Router, routing::{post, get, put, delete}};
use sqlx::PgPool;

mod health;
mod entries;
mod auth;

pub fn router(pool: PgPool) -> Router {
    Router::new()
        .route("/entries", post(entries::create))
        .route("/entries", get(entries::list))
        .route("/entries/:id", get(entries::get))
        .route("/entries/:id", put(entries::update))
        .route("/entries/:id", delete(entries::delete))
        .route("/auth/register", post(auth::register))
        .route("/auth/login", post(auth::login))
        .with_state(pool)
}
```

Create `src/api/health.rs`:

```rust
use axum::Json;
use serde_json::json;

pub async fn handler() -> Json<serde_json::Value> {
    Json(json!({ "status": "healthy", "service": "granate" }))
}
```

Create `src/api/auth.rs`:

```rust
use axum::{Json, extract::State};
use sqlx::PgPool;
use serde::Deserialize;
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use argon2::password_hash::{rand_core::OsRng, SaltString};
use jsonwebtoken::{encode, Header, EncodingKey};
use chrono::{Utc, Duration};
use crate::{models::User, error::AppError};

#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    email: String,
    password: String,
    name: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    email: String,
    password: String,
}

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    token: String,
    user_id: String,
}

pub async fn register(
    State(pool): State<PgPool>,
    Json(req): Json<RegisterRequest>,
) -> Result<Json<AuthResponse>, AppError> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2.hash_password(req.password.as_bytes(), &salt)
        .map_err(|_| AppError::Validation("Failed to hash password".to_string()))?
        .to_string();
    
    let user = sqlx::query_as::<_, User>(
        "INSERT INTO users (email, password_hash, name) VALUES ($1, $2, $3) RETURNING *"
    )
    .bind(&req.email)
    .bind(&password_hash)
    .bind(&req.name)
    .fetch_one(&pool)
    .await?;
    
    let token = create_jwt(&user.id.to_string())?;
    
    Ok(Json(AuthResponse {
        token,
        user_id: user.id.to_string(),
    }))
}

fn create_jwt(user_id: &str) -> Result<String, AppError> {
    let jwt_secret = std::env::var("JWT_SECRET").unwrap();
    let claims = serde_json::json!({
        "sub": user_id,
        "exp": (Utc::now() + Duration::hours(24)).timestamp(),
    });
    
    encode(&Header::default(), &claims, &EncodingKey::from_secret(jwt_secret.as_bytes()))
        .map_err(|_| AppError::Validation("Token creation failed".to_string()))
}

// Similar for login endpoint...
```

## Phase 4: Run instructions

```bash
# Start PostgreSQL
docker-compose up -d

# Run migrations (after adding sqlx-cli)
cargo install sqlx-cli
sqlx migrate run

# Start the server
cargo run

# Test health endpoint
curl http://localhost:3000/health

# Test creating an entry
curl -X POST http://localhost:3000/api/v1/entries \
  -H "Content-Type: application/json" \
  -d '{"content_type_id":"...","slug":"hello-world","data":{"title":"Hello","body":"World"}}'
```

### Next features to implement (after basic structure):

5. Content type schema validation against JSONB data
6. GraphQL layer with async-graphql
7. File upload to S3/local storage
8. Webhook triggers on publish
9. API keys for frontend clients
10. Preview tokens for drafts
```

