use axum::{Router, routing::{post, get, put, delete}};

pub mod health;
pub mod entries;
pub mod content_types;
pub mod tags;
pub mod auth;
pub mod media;
pub mod products;

#[cfg(test)]
mod products_test;

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
        .route("/media/upload", post(media::upload))
        .route("/media", get(media::list_media))
        .route("/auth/me", get(auth::me))
        .route("/health", get(health::handler))
        .route("/products", get(products::list).post(products::create))
        .route("/products/{slug}", get(products::get_by_slug))
        .route("/products/{id}", put(products::update).delete(products::delete))
        .route("/products/{id}/variants", post(products::add_variant))
        .route("/products/{id}/variants/{variant_id}", delete(products::remove_variant))
}
