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
