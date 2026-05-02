mod config;
mod db;
mod models;
mod api;
mod error;
mod middleware;

use axum::{Router, routing::get, extract::FromRef};
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;
use std::net::SocketAddr;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use sqlx::PgPool;
use middleware::JwksState;

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub jwks: JwksState,
}

impl FromRef<AppState> for PgPool {
    fn from_ref(state: &AppState) -> Self {
        state.pool.clone()
    }
}

impl FromRef<AppState> for JwksState {
    fn from_ref(state: &AppState) -> Self {
        state.jwks.clone()
    }
}

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
    
    let jwks = JwksState::new(config.mangosteen_jwks_url.clone());
    
    // Initial JWKS fetch
    if let Err(e) = jwks.refresh_key().await {
        tracing::warn!("Failed to fetch initial JWKS key: {}. Will retry on first request.", e);
    }
    
    let state = AppState { pool, jwks };
    
    let app = Router::new()
        .route("/health", get(api::health::handler))
        .nest("/api/v1", api::router(state))
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http());
    
    let addr = SocketAddr::from(([0, 0, 0, 0], config.port));
    tracing::info!("Granate CMS starting on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;
    
    Ok(())
}
