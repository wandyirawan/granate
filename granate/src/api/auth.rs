use axum::{Json, extract::State};
use sqlx::PgPool;
use serde::{Deserialize, Serialize};
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use argon2::password_hash::{rand_core::OsRng, SaltString};
use jsonwebtoken::{encode, Header, EncodingKey};
use chrono::{Utc, Duration};
use crate::{models::User, error::AppError};

#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    email: String,
    password: String,
    name: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    email: String,
    password: String,
}

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    token: String,
    user_id: String,
}

pub async fn register(
    State(pool): State<PgPool>,
    Json(req): Json<RegisterRequest>,
) -> Result<Json<AuthResponse>, AppError> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2.hash_password(req.password.as_bytes(), &salt)
        .map_err(|_| AppError::Validation("Failed to hash password".to_string()))?
        .to_string();
    
    let user = sqlx::query_as::<_, User>(
        "INSERT INTO users (email, password_hash, name) VALUES ($1, $2, $3) RETURNING *"
    )
    .bind(&req.email)
    .bind(&password_hash)
    .bind(&req.name)
    .fetch_one(&pool)
    .await?;
    
    let token = create_jwt(&user.id.to_string())?;
    
    Ok(Json(AuthResponse {
        token,
        user_id: user.id.to_string(),
    }))
}

pub async fn login(
    State(pool): State<PgPool>,
    Json(req): Json<LoginRequest>,
) -> Result<Json<AuthResponse>, AppError> {
    let user = sqlx::query_as::<_, User>(
        "SELECT * FROM users WHERE email = $1"
    )
    .bind(&req.email)
    .fetch_optional(&pool)
    .await?
    .ok_or(AppError::Unauthorized)?;
    
    let parsed_hash = PasswordHash::new(&user.password_hash)
        .map_err(|_| AppError::Validation("Invalid password hash".to_string()))?;
    
    let argon2 = Argon2::default();
    argon2.verify_password(req.password.as_bytes(), &parsed_hash)
        .map_err(|_| AppError::Unauthorized)?;
    
    let token = create_jwt(&user.id.to_string())?;
    
    Ok(Json(AuthResponse {
        token,
        user_id: user.id.to_string(),
    }))
}

fn create_jwt(user_id: &str) -> Result<String, AppError> {
    let jwt_secret = std::env::var("JWT_SECRET").unwrap();
    let claims = serde_json::json!({
        "sub": user_id,
        "exp": (Utc::now() + Duration::hours(24)).timestamp(),
    });
    
    encode(&Header::default(), &claims, &EncodingKey::from_secret(jwt_secret.as_bytes()))
        .map_err(|_| AppError::Validation("Token creation failed".to_string()))
}
use axum::{Json, extract::State};
use sqlx::PgPool;
use serde::{Deserialize, Serialize};
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use argon2::password_hash::{rand_core::OsRng, SaltString};
use jsonwebtoken::{encode, Header, EncodingKey};
use chrono::{Utc, Duration};
use crate::{models::User, error::AppError};

#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    email: String,
    password: String,
    name: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    email: String,
    password: String,
}

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    token: String,
    user_id: String,
}

pub async fn register(
    State(pool): State<PgPool>,
    Json(req): Json<RegisterRequest>,
) -> Result<Json<AuthResponse>, AppError> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2.hash_password(req.password.as_bytes(), &salt)
        .map_err(|_| AppError::Validation("Failed to hash password".to_string()))?
        .to_string();
    
    let user = sqlx::query_as::<_, User>(
        "INSERT INTO users (email, password_hash, name) VALUES ($1, $2, $3) RETURNING *"
    )
    .bind(&req.email)
    .bind(&password_hash)
    .bind(&req.name)
    .fetch_one(&pool)
    .await?;
    
    let token = create_jwt(&user.id.to_string())?;
    
    Ok(Json(AuthResponse {
        token,
        user_id: user.id.to_string(),
    }))
}

pub async fn login(
    State(pool): State<PgPool>,
    Json(req): Json<LoginRequest>,
) -> Result<Json<AuthResponse>, AppError> {
    let user = sqlx::query_as::<_, User>(
        "SELECT * FROM users WHERE email = $1"
    )
    .bind(&req.email)
    .fetch_optional(&pool)
    .await?
    .ok_or(AppError::Unauthorized)?;
    
    let parsed_hash = PasswordHash::new(&user.password_hash)
        .map_err(|_| AppError::Validation("Invalid password hash".to_string()))?;
    
    let argon2 = Argon2::default();
    argon2.verify_password(req.password.as_bytes(), &parsed_hash)
        .map_err(|_| AppError::Unauthorized)?;
    
    let token = create_jwt(&user.id.to_string())?;
    
    Ok(Json(AuthResponse {
        token,
        user_id: user.id.to_string(),
    }))
}

fn create_jwt(user_id: &str) -> Result<String, AppError> {
    let jwt_secret = std::env::var("JWT_SECRET").unwrap();
    let claims = serde_json::json!({
        "sub": user_id,
        "exp": (Utc::now() + Duration::hours(24)).timestamp(),
    });
    
    encode(&Header::default(), &claims, &EncodingKey::from_secret(jwt_secret.as_bytes()))
        .map_err(|_| AppError::Validation("Token creation failed".to_string()))
}
use axum::{Json, extract::State};
use sqlx::PgPool;
use serde::{Deserialize, Serialize};
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use argon2::password_hash::{rand_core::OsRng, SaltString};
use jsonwebtoken::{encode, Header, EncodingKey};
use chrono::{Utc, Duration};
use crate::{models::User, error::AppError};

#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    email: String,
    password: String,
    name: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    email: String,
    password: String,
}

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    token: String,
    user_id: String,
}

pub async fn register(
    State(pool): State<PgPool>,
    Json(req): Json<RegisterRequest>,
) -> Result<Json<AuthResponse>, AppError> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2.hash_password(req.password.as_bytes(), &salt)
        .map_err(|_| AppError::Validation("Failed to hash password".to_string()))?
        .to_string();
    
    let user = sqlx::query_as::<_, User>(
        "INSERT INTO users (email, password_hash, name) VALUES ($1, $2, $3) RETURNING *"
    )
    .bind(&req.email)
    .bind(&password_hash)
    .bind(&req.name)
    .fetch_one(&pool)
    .await?;
    
    let token = create_jwt(&user.id.to_string())?;
    
    Ok(Json(AuthResponse {
        token,
        user_id: user.id.to_string(),
    }))
}

pub async fn login(
    State(pool): State<PgPool>,
    Json(req): Json<LoginRequest>,
) -> Result<Json<AuthResponse>, AppError> {
    let user = sqlx::query_as::<_, User>(
        "SELECT * FROM users WHERE email = $1"
    )
    .bind(&req.email)
    .fetch_optional(&pool)
    .await?
    .ok_or(AppError::Unauthorized)?;
    
    let parsed_hash = PasswordHash::new(&user.password_hash)
        .map_err(|_| AppError::Validation("Invalid password hash".to_string()))?;
    
    let argon2 = Argon2::default();
    argon2.verify_password(req.password.as_bytes(), &parsed_hash)
        .map_err(|_| AppError::Unauthorized)?;
    
    let token = create_jwt(&user.id.to_string())?;
    
    Ok(Json(AuthResponse {
        token,
        user_id: user.id.to_string(),
    }))
}

fn create_jwt(user_id: &str) -> Result<String, AppError> {
    let jwt_secret = std::env::var("JWT_SECRET").unwrap();
    let claims = serde_json::json!({
        "sub": user_id,
        "exp": (Utc::now() + Duration::hours(24)).timestamp(),
    });
    
    encode(&Header::default(), &claims, &EncodingKey::from_secret(jwt_secret.as_bytes()))
        .map_err(|_| AppError::Validation("Token creation failed".to_string()))
}
