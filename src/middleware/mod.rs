use serde::Deserialize;

pub mod jwt;

#[derive(Debug, Deserialize, Clone)]
#[allow(dead_code)]
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
    type Rejection = axum::http::StatusCode;

    async fn from_request_parts(parts: &mut axum::http::request::Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let claims = parts.extensions.get::<Claims>().cloned();
        match claims {
            Some(claims) => Ok(AuthenticatedUser(claims)),
            None => Err(axum::http::StatusCode::UNAUTHORIZED),
        }
    }
}
