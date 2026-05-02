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
