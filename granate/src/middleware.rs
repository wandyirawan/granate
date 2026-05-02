use axum::{
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
    response::{IntoResponse, Response},
    Json,
};
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JwtClaims {
    pub sub: String,
    pub name: String,
    pub email: String,
    pub role: String,
    pub exp: usize,
}

#[derive(Debug, Clone)]
pub struct AuthenticatedUser {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub role: String,
}

#[derive(Debug, Clone)]
pub struct JwksState {
    pub client: Client,
    pub jwks_url: String,
    pub public_key: Arc<RwLock<Option<DecodingKey>>>,
}

impl JwksState {
    pub fn new(jwks_url: String) -> Self {
        Self {
            client: Client::new(),
            jwks_url,
            public_key: Arc::new(RwLock::new(None)),
        }
    }

    pub async fn refresh_key(&self) -> Result<(), String> {
        let resp = self
            .client
            .get(&self.jwks_url)
            .send()
            .await
            .map_err(|e| format!("Failed to fetch JWKS: {}", e))?;

        let jwks: serde_json::Value = resp
            .json()
            .await
            .map_err(|e| format!("Failed to parse JWKS: {}", e))?;

        let keys = jwks["keys"]
            .as_array()
            .ok_or("No keys in JWKS response")?;

        if keys.is_empty() {
            return Err("Empty JWKS keys array".to_string());
        }

        let first_key = &keys[0];
        let n = first_key["n"]
            .as_str()
            .ok_or("Missing 'n' in JWK")?;
        let e = first_key["e"]
            .as_str()
            .ok_or("Missing 'e' in JWK")?;

        let decoding_key = DecodingKey::from_rsa_components(n, e)
            .map_err(|_| "Failed to create RSA key from components".to_string())?;

        let mut key = self.public_key.write().await;
        *key = Some(decoding_key);
        Ok(())
    }
}

impl<S> FromRequestParts<S> for AuthenticatedUser
where
    S: Send + Sync,
    JwksState: axum::extract::FromRef<S>,
{
    type Rejection = Response;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let jwks_state = JwksState::from_ref(state);

        let auth_header = parts
            .headers
            .get("Authorization")
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.strip_prefix("Bearer "))
            .ok_or_else(|| {
                (
                    StatusCode::UNAUTHORIZED,
                    Json(json!({"error": "Missing or invalid Authorization header"})),
                )
                    .into_response()
            })?;

        let key = jwks_state.public_key.read().await;
        let decoding_key = key.as_ref().ok_or_else(|| {
            (
                StatusCode::UNAUTHORIZED,
                Json(json!({"error": "JWKS key not loaded"})),
            )
                .into_response()
        })?;

        let mut validation = Validation::new(Algorithm::RS256);
        validation.validate_exp = true;

        let token_data = decode::<JwtClaims>(auth_header, decoding_key, &validation)
            .map_err(|e| {
                tracing::warn!("JWT validation failed: {}", e);
                (
                    StatusCode::UNAUTHORIZED,
                    Json(json!({"error": "Invalid or expired token"})),
                )
                    .into_response()
            })?;

        let claims = token_data.claims;

        let user_id = Uuid::parse_str(&claims.sub).map_err(|_| {
            (
                StatusCode::UNAUTHORIZED,
                Json(json!({"error": "Invalid user ID in token"})),
            )
                .into_response()
        })?;

        Ok(AuthenticatedUser {
            id: user_id,
            name: claims.name,
            email: claims.email,
            role: claims.role,
        })
    }
}
