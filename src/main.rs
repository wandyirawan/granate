mod config;
mod db;
mod models;
mod api;
mod error;
mod middleware;

use axum::{Router, routing::get, Extension};
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

    dotenvy::dotenv().ok();
    let config = config::Config::from_env()?;
    
    let pool = db::create_pool(&config.database_url).await?;
    sqlx::migrate!().run(&pool).await?;
    
    let pem = config.mangosteen_jwt_public_key.replace("\\n", "\n");
    let decoding_key = jsonwebtoken::DecodingKey::from_rsa_pem(pem.as_bytes())?;
    
    let auth_state = Arc::new(middleware::AuthState { decoding_key });
    
    let api_routes = api::router()
        .layer(axum::middleware::from_fn(middleware::auth_middleware))
        .layer(Extension(auth_state))
        .layer(Extension(pool.clone()));
    
    let app = Router::new()
        .route("/health", get(api::health::handler))
        .nest("/api/v1", api_routes)
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http());
    
    let addr = SocketAddr::from(([0, 0, 0, 0], config.port));
    tracing::info!("Granate CMS starting on {}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    
    Ok(())
}
