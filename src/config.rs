#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub mangosteen_url: String,
    pub mangosteen_jwks_url: String,
    pub minio_endpoint: String,
    pub minio_access_key: String,
    pub minio_secret_key: String,
    pub minio_bucket: String,
    pub port: u16,
}

impl Config {
    pub fn from_env() -> anyhow::Result<Self> {
        let database_url = std::env::var("DATABASE_URL")
            .map_err(|_| anyhow::anyhow!("DATABASE_URL not found. Create a .env file or export the variable."))?;
        
        let mangosteen_url = std::env::var("MANGOSTEEN_URL")
            .map_err(|_| anyhow::anyhow!("MANGOSTEEN_URL not found. Create a .env file or export the variable."))?;
        
        let mangosteen_jwks_url = std::env::var("MANGOSTEEN_JWKS_URL")
            .map_err(|_| anyhow::anyhow!("MANGOSTEEN_JWKS_URL not found. Create a .env file or export the variable."))?;
        
        Ok(Config {
            database_url,
            mangosteen_url,
            mangosteen_jwks_url,
            minio_endpoint: std::env::var("MINIO_ENDPOINT").unwrap_or_else(|_| "localhost:9000".into()),
            minio_access_key: std::env::var("MINIO_ACCESS_KEY").unwrap_or_else(|_| "pomegranate".into()),
            minio_secret_key: std::env::var("MINIO_SECRET_KEY").unwrap_or_else(|_| "pomegranate123".into()),
            minio_bucket: std::env::var("MINIO_BUCKET").unwrap_or_else(|_| "granate-media".into()),
            port: std::env::var("PORT")
                .unwrap_or_else(|_| "3000".to_string())
                .parse()?,
        })
    }
}
