use axum::{Router, routing::{post, get, put, delete}};

pub mod health;
mod entries;
mod content_types;
mod tags;
pub mod auth;

pub fn router() -> Router {
    Router::new()
        .route("/entries", post(entries::create))
        .route("/entries", get(entries::list))
        .route("/entries/{id}", get(entries::get))
        .route("/entries/{id}", put(entries::update))
        .route("/entries/{id}", delete(entries::delete))
        .route("/content-types", post(content_types::create))
        .route("/content-types", get(content_types::list))
        .route("/content-types/{id}", get(content_types::get))
        .route("/content-types/{id}", put(content_types::update))
        .route("/content-types/{id}", delete(content_types::delete))
        .route("/tags", post(tags::create))
        .route("/tags", get(tags::list))
        .route("/tags/{id}", delete(tags::delete))
        .route("/auth/me", get(auth::me))
        .route("/health", get(health::handler))
}
