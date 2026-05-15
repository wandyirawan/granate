mod config;
mod db;
mod models;
mod api;
mod error;
mod middleware;
mod media;

use axum::{Router, routing::get, extract::Extension};
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

    // Ensure Minio bucket exists
    {
        let s3_config = aws_sdk_s3::Config::builder()
            .credentials_provider(aws_sdk_s3::config::Credentials::new(
                &config.minio_access_key,
                &config.minio_secret_key,
                None,
                None,
                "minio",
            ))
            .region(aws_sdk_s3::config::Region::new("us-east-1"))
            .endpoint_url(format!("http://{}", config.minio_endpoint))
            .force_path_style(true)
            .build();

        let client = aws_sdk_s3::Client::from_conf(s3_config);

        match client.head_bucket().bucket(&config.minio_bucket).send().await {
            Ok(_) => tracing::info!("Minio bucket '{}' ready", config.minio_bucket),
            Err(_) => {
                tracing::info!("Creating Minio bucket '{}'", config.minio_bucket);
                client.create_bucket().bucket(&config.minio_bucket).send().await?;
                tracing::info!("Minio bucket '{}' created", config.minio_bucket);
            }
        }
    }

    // Public routes (no auth required)
    let public_routes = Router::new()
        .route("/auth/login", axum::routing::post(api::auth::login))
        .route("/auth/register", axum::routing::post(api::auth::register));

    // Protected routes (auth required)
    let protected_routes = api::router()
        .layer(axum::middleware::from_fn(middleware::auth_middleware))
        .layer(Extension(auth_state));

    let app = Router::new()
        .route("/health", get(api::health::handler))
        .nest("/api/v1/auth", public_routes)
        .nest("/api/v1", protected_routes)
        .layer(Extension(pool.clone()))
        .layer(Extension(config.clone()))
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http());

    let addr = SocketAddr::from(([0, 0, 0, 0], config.port));
    tracing::info!("Granate CMS starting on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
