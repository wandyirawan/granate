#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub mangosteen_jwt_public_key: String,
    pub port: u16,
}

impl Config {
    pub fn from_env() -> anyhow::Result<Self> {
        let database_url = std::env::var("DATABASE_URL")
            .map_err(|_| anyhow::anyhow!("DATABASE_URL not found. Create a .env file or export the variable."))?;
        
        let key_file = std::env::var("MANGOSTEEN_JWT_PUBLIC_KEY_FILE")
            .map_err(|_| anyhow::anyhow!("MANGOSTEEN_JWT_PUBLIC_KEY_FILE not found. Create a .env file or export the variable."))?;
        
        let mangosteen_jwt_public_key = std::fs::read_to_string(&key_file)
            .map_err(|e| anyhow::anyhow!("Failed to read public key file '{}': {}", key_file, e))?;
        
        Ok(Config {
            database_url,
            mangosteen_jwt_public_key,
            port: std::env::var("PORT")
                .unwrap_or_else(|_| "3000".to_string())
                .parse()?,
        })
    }
}
