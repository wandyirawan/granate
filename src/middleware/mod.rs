use axum::{
    extract::Extension,
    http::StatusCode,
    middleware::Next,
    response::Response,
    body::Body,
};
use axum::http::Request;
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};
use serde::Deserialize;
use std::sync::Arc;

pub mod jwt;

#[derive(Debug, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,
    pub email: String,
    pub role: String,
    #[allow(dead_code)]
    pub exp: usize,
}

#[derive(Clone)]
pub struct AuthState {
    pub jwks: jsonwebtoken::jwk::JwkSet,
}

pub use jwt::jwt_middleware as auth_middleware;

pub struct AuthenticatedUser(pub Claims);

impl<S: Send + Sync> axum::extract::FromRequestParts<S> for AuthenticatedUser {
    type Rejection = StatusCode;

    async fn from_request_parts(parts: &mut axum::http::request::Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let claims = parts.extensions.get::<Claims>().cloned();
        match claims {
            Some(claims) => Ok(AuthenticatedUser(claims)),
            None => Err(StatusCode::UNAUTHORIZED),
        }
    }
}
