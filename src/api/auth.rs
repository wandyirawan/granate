use axum::{
    extract::{Extension, Json},
    http::{HeaderMap, StatusCode},
    response::Json as ResponseJson,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Deserialize, Serialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RegisterRequest {
    pub email: String,
    pub password: String,
    pub name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserInfo {
    pub id: String,
    pub email: String,
    pub role: String,
    pub active: bool,
}

pub async fn login(
    Extension(config): Extension<Arc<crate::config::Config>>,
    Json(payload): Json<LoginRequest>,
) -> Result<ResponseJson<LoginResponse>, StatusCode> {
    let client = reqwest::Client::new();
    let mangosteen_url = format!("{}/api/auth/login", config.mangosteen_url);
    
    let response = client
        .post(&mangosteen_url)
        .json(&payload)
        .send()
        .await
        .map_err(|e| {
            tracing::error!("Failed to call Mangosteen login: {}", e);
            StatusCode::BAD_GATEWAY
        })?;
    
    if !response.status().is_success() {
        let status = StatusCode::from_u16(response.status().as_u16())
            .unwrap_or(StatusCode::BAD_GATEWAY);
        return Err(status);
    }
    
    let login_response: LoginResponse = response.json().await.map_err(|e| {
        tracing::error!("Failed to parse Mangosteen login response: {}", e);
        StatusCode::BAD_GATEWAY
    })?;
    
    Ok(ResponseJson(login_response))
}

pub async fn me(
    Extension(config): Extension<Arc<crate::config::Config>>,
    headers: HeaderMap,
) -> Result<ResponseJson<UserInfo>, StatusCode> {
    let auth_header = headers
        .get("authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|h| h.strip_prefix("Bearer "))
        .ok_or(StatusCode::UNAUTHORIZED)?;
    
    let client = reqwest::Client::new();
    let mangosteen_url = format!("{}/api/users/me", config.mangosteen_url);
    
    let response = client
        .get(&mangosteen_url)
        .header("Authorization", format!("Bearer {}", auth_header))
        .send()
        .await
        .map_err(|e| {
            tracing::error!("Failed to call Mangosteen user info: {}", e);
            StatusCode::BAD_GATEWAY
        })?;
    
    if !response.status().is_success() {
        return Err(StatusCode::from_u16(response.status().as_u16())
            .unwrap_or(StatusCode::BAD_GATEWAY));
    }
    
    let user_info: UserInfo = response.json().await.map_err(|e| {
        tracing::error!("Failed to parse Mangosteen user info: {}", e);
        StatusCode::BAD_GATEWAY
    })?;
    
    Ok(ResponseJson(user_info))
}

pub async fn register(
    Extension(config): Extension<Arc<crate::config::Config>>,
    Json(payload): Json<RegisterRequest>,
) -> Result<ResponseJson<LoginResponse>, StatusCode> {
    let client = reqwest::Client::new();
    let mangosteen_url = format!("{}/api/auth/register", config.mangosteen_url);
    
    let response = client
        .post(&mangosteen_url)
        .json(&payload)
        .send()
        .await
        .map_err(|e| {
            tracing::error!("Failed to call Mangosteen register: {}", e);
            StatusCode::BAD_GATEWAY
        })?;
    
    if !response.status().is_success() {
        let status = StatusCode::from_u16(response.status().as_u16())
            .unwrap_or(StatusCode::BAD_GATEWAY);
        return Err(status);
    }
    
    let login_response: LoginResponse = response.json().await.map_err(|e| {
        tracing::error!("Failed to parse Mangosteen register response: {}", e);
        StatusCode::BAD_GATEWAY
    })?;
    
    Ok(ResponseJson(login_response))
}
