use axum::{
    extract::Extension,
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};
use serde::Deserialize;
use std::sync::Arc;
use async_trait::async_trait;

#[derive(Debug, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

#[derive(Clone)]
pub struct AuthState {
    pub decoding_key: DecodingKey,
}

pub async fn auth_middleware<B>(
    Extension(auth_state): Extension<Arc<AuthState>>,
    mut request: Request<B>,
    next: Next<B>,
) -> Result<Response, StatusCode> {
    let auth_header = request
        .headers()
        .get("authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|h| h.strip_prefix("Bearer "));
    
    let token = auth_header.ok_or(StatusCode::UNAUTHORIZED)?;
    
    let validation = Validation::new(Algorithm::RS256);
    
    let token_data = decode::<Claims>(token, &auth_state.decoding_key, &validation)
        .map_err(|_| StatusCode::UNAUTHORIZED)?;
    
    request.extensions_mut().insert(token_data.claims.sub);
    Ok(next.run(request).await)
}

pub struct AuthUser(pub String);

#[async_trait]
impl<S: Send + Sync> axum::extract::FromRequestParts<S> for AuthUser {
    type Rejection = StatusCode;

    async fn from_request_parts(parts: &mut axum::http::request::Parts, _state: &S) -> Result<Self, StatusCode> {
        let user_id = parts.extensions.get::<String>().cloned();
        match user_id {
            Some(id) => Ok(AuthUser(id)),
            None => Err(StatusCode::UNAUTHORIZED),
        }
    }
}
