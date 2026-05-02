use axum::{Router, routing::{post, get, put, delete}};
use crate::AppState;

mod health;
mod entries;

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/entries", post(entries::create))
        .route("/entries", get(entries::list))
        .route("/entries/:id", get(entries::get))
        .route("/entries/:id", put(entries::update))
        .route("/entries/:id", delete(entries::delete))
        .with_state(state)
}
