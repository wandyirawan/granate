use anyhow::Context;
use axum::{
    extract::Extension,
    http::StatusCode,
    middleware::Next,
    response::Response,
    body::Body,
    http::Request,
};
use jsonwebtoken::{decode, decode_header, DecodingKey, Validation, jwk::JwkSet};
use std::sync::Arc;

pub async fn fetch_jwks(url: &str) -> anyhow::Result<JwkSet> {
    let response = reqwest::get(url)
        .await
        .context("Failed to fetch JWKS")?;
    let jwks: JwkSet = response
        .json()
        .await
        .context("Failed to parse JWKS")?;
    Ok(jwks)
}

pub async fn jwt_middleware(
    Extension(auth_state): Extension<Arc<super::AuthState>>,
    mut request: Request<Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    let auth_header = request
        .headers()
        .get("authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|h| h.strip_prefix("Bearer "));
    
    let token = auth_header.ok_or(StatusCode::UNAUTHORIZED)?;
    
    let header = decode_header(token)
        .map_err(|e| {
            tracing::warn!("Failed to decode token header: {}", e);
            StatusCode::UNAUTHORIZED
        })?;
    
    let kid = header.kid.ok_or_else(|| {
        tracing::warn!("No kid in token header");
        StatusCode::UNAUTHORIZED
    })?;
    
    let jwk = auth_state.jwks.find(&kid).ok_or_else(|| {
        tracing::warn!("No matching kid found in JWKS");
        StatusCode::UNAUTHORIZED
    })?;
    
    let decoding_key = DecodingKey::from_jwk(jwk)
        .map_err(|e| {
            tracing::warn!("Failed to convert JWK to DecodingKey: {}", e);
            StatusCode::UNAUTHORIZED
        })?;
    
    let validation = Validation::new(jsonwebtoken::Algorithm::RS256);
    let token_data = decode::<super::Claims>(token, &decoding_key, &validation)
        .map_err(|e| {
            tracing::warn!("Token validation failed: {}", e);
            StatusCode::UNAUTHORIZED
        })?;
    
    request.extensions_mut().insert(token_data.claims);
    Ok(next.run(request).await)
}
