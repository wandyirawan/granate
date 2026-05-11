mod config;
mod db;
mod models;
mod api;
mod error;
mod middleware;

use axum::{Router, routing::{post, get, put, delete}, extract::Extension};
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;
use std::net::SocketAddr;
use std::sync::Arc;
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

    if let Err(e) = dotenvy::dotenv() {
        tracing::warn!(".env file not loaded: {}. Using existing environment variables.", e);
    }
    
    let config = config::Config::from_env()?;
    let config = Arc::new(config);
    
    let pool = db::create_pool(&config.database_url).await?;
    sqlx::migrate!().run(&pool).await?;
    
    let jwks = middleware::jwt::fetch_jwks(&config.mangosteen_jwks_url).await?;
    let auth_state = Arc::new(middleware::AuthState { jwks });
    
    // Public routes (no auth required)
    let public_routes = Router::new()
        .route("/auth/login", post(api::auth::login))
        .route("/auth/register", post(api::auth::register));
    
    // Protected routes (auth required)
    let protected_routes = api::router()
        .layer(axum::middleware::from_fn(middleware::auth_middleware))
        .layer(Extension(auth_state))
        .layer(Extension(config.clone()));
    
    let app = Router::new()
        .route("/health", get(api::health::handler))
        .nest("/api/v1/auth", public_routes)
        .nest("/api/v1", protected_routes)
        .layer(Extension(pool.clone()))
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http());
    
    let addr = SocketAddr::from(([0, 0, 0, 0], config.port));
    tracing::info!("Granate CMS starting on {}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    
    Ok(())
}
