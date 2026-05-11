#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub mangosteen_url: String,
    pub mangosteen_jwks_url: String,
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
            port: std::env::var("PORT")
                .unwrap_or_else(|_| "3000".to_string())
                .parse()?,
        })
    }
}
