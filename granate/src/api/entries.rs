use axum::{Json, extract::{State, Path}, http::StatusCode};
use sqlx::PgPool;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use serde_json::Value as JsonValue;
use crate::{models::Entry, error::AppError};

#[derive(Debug, Deserialize)]
pub struct CreateEntryRequest {
    pub content_type_id: Uuid,
    pub author_id: Uuid,
    pub slug: String,
    pub data: JsonValue,
    pub meta: Option<JsonValue>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateEntryRequest {
    pub slug: Option<String>,
    pub status: Option<String>,
    pub data: Option<JsonValue>,
    pub meta: Option<JsonValue>,
}

#[derive(Debug, Serialize)]
pub struct EntryResponse {
    pub id: Uuid,
    pub content_type_id: Uuid,
    pub author_id: Uuid,
    pub slug: String,
    pub status: String,
    pub data: JsonValue,
    pub meta: JsonValue,
    pub published_at: Option<chrono::DateTime<chrono::Utc>>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl From<Entry> for EntryResponse {
    fn from(entry: Entry) -> Self {
        EntryResponse {
            id: entry.id,
            content_type_id: entry.content_type_id,
            author_id: entry.author_id,
            slug: entry.slug,
            status: entry.status,
            data: entry.data,
            meta: entry.meta,
            published_at: entry.published_at,
            created_at: entry.created_at,
            updated_at: entry.updated_at,
        }
    }
}

pub async fn create(
    State(pool): State<PgPool>,
    Json(req): Json<CreateEntryRequest>,
) -> Result<(StatusCode, Json<EntryResponse>), AppError> {
    let meta = req.meta.unwrap_or(serde_json::json!({}));
    
    let entry = sqlx::query_as::<_, Entry>(
        "INSERT INTO entries (content_type_id, author_id, slug, data, meta) VALUES ($1, $2, $3, $4, $5) RETURNING *"
    )
    .bind(req.content_type_id)
    .bind(req.author_id)
    .bind(&req.slug)
    .bind(&req.data)
    .bind(&meta)
    .fetch_one(&pool)
    .await?;
    
    Ok((StatusCode::CREATED, Json(EntryResponse::from(entry))))
}

pub async fn list(
    State(pool): State<PgPool>,
) -> Result<Json<Vec<EntryResponse>>, AppError> {
    let entries = sqlx::query_as::<_, Entry>(
        "SELECT * FROM entries ORDER BY created_at DESC"
    )
    .fetch_all(&pool)
    .await?;
    
    Ok(Json(entries.into_iter().map(EntryResponse::from).collect()))
}

pub async fn get(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<Json<EntryResponse>, AppError> {
    let entry = sqlx::query_as::<_, Entry>(
        "SELECT * FROM entries WHERE id = $1"
    )
    .bind(id)
    .fetch_optional(&pool)
    .await?
    .ok_or(AppError::NotFound)?;
    
    Ok(Json(EntryResponse::from(entry)))
}

pub async fn update(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateEntryRequest>,
) -> Result<Json<EntryResponse>, AppError> {
    let existing = sqlx::query_as::<_, Entry>(
        "SELECT * FROM entries WHERE id = $1"
    )
    .bind(id)
    .fetch_optional(&pool)
    .await?
    .ok_or(AppError::NotFound)?;
    
    let slug = req.slug.unwrap_or(existing.slug);
    let status = req.status.unwrap_or(existing.status);
    let data = req.data.unwrap_or(existing.data);
    let meta = req.meta.unwrap_or(existing.meta);
    
    let entry = sqlx::query_as::<_, Entry>(
        "UPDATE entries SET slug = $1, status = $2, data = $3, meta = $4 WHERE id = $5 RETURNING *"
    )
    .bind(&slug)
    .bind(&status)
    .bind(&data)
    .bind(&meta)
    .bind(id)
    .fetch_one(&pool)
    .await?;
    
    Ok(Json(EntryResponse::from(entry)))
}

pub async fn delete(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    let result = sqlx::query("DELETE FROM entries WHERE id = $1")
        .bind(id)
        .execute(&pool)
        .await?;
    
    if result.rows_affected() == 0 {
        return Err(AppError::NotFound);
    }
    
    Ok(StatusCode::NO_CONTENT)
}
use axum::{Json, extract::{State, Path}, http::StatusCode};
use sqlx::PgPool;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use serde_json::Value as JsonValue;
use crate::{models::Entry, error::AppError};

#[derive(Debug, Deserialize)]
pub struct CreateEntryRequest {
    pub content_type_id: Uuid,
    pub author_id: Uuid,
    pub slug: String,
    pub data: JsonValue,
    pub meta: Option<JsonValue>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateEntryRequest {
    pub slug: Option<String>,
    pub status: Option<String>,
    pub data: Option<JsonValue>,
    pub meta: Option<JsonValue>,
}

#[derive(Debug, Serialize)]
pub struct EntryResponse {
    pub id: Uuid,
    pub content_type_id: Uuid,
    pub author_id: Uuid,
    pub slug: String,
    pub status: String,
    pub data: JsonValue,
    pub meta: JsonValue,
    pub published_at: Option<chrono::DateTime<chrono::Utc>>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl From<Entry> for EntryResponse {
    fn from(entry: Entry) -> Self {
        EntryResponse {
            id: entry.id,
            content_type_id: entry.content_type_id,
            author_id: entry.author_id,
            slug: entry.slug,
            status: entry.status,
            data: entry.data,
            meta: entry.meta,
            published_at: entry.published_at,
            created_at: entry.created_at,
            updated_at: entry.updated_at,
        }
    }
}

pub async fn create(
    State(pool): State<PgPool>,
    Json(req): Json<CreateEntryRequest>,
) -> Result<(StatusCode, Json<EntryResponse>), AppError> {
    let meta = req.meta.unwrap_or(serde_json::json!({}));
    
    let entry = sqlx::query_as::<_, Entry>(
        "INSERT INTO entries (content_type_id, author_id, slug, data, meta) VALUES ($1, $2, $3, $4, $5) RETURNING *"
    )
    .bind(req.content_type_id)
    .bind(req.author_id)
    .bind(&req.slug)
    .bind(&req.data)
    .bind(&meta)
    .fetch_one(&pool)
    .await?;
    
    Ok((StatusCode::CREATED, Json(EntryResponse::from(entry))))
}

pub async fn list(
    State(pool): State<PgPool>,
) -> Result<Json<Vec<EntryResponse>>, AppError> {
    let entries = sqlx::query_as::<_, Entry>(
        "SELECT * FROM entries ORDER BY created_at DESC"
    )
    .fetch_all(&pool)
    .await?;
    
    Ok(Json(entries.into_iter().map(EntryResponse::from).collect()))
}

pub async fn get(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<Json<EntryResponse>, AppError> {
    let entry = sqlx::query_as::<_, Entry>(
        "SELECT * FROM entries WHERE id = $1"
    )
    .bind(id)
    .fetch_optional(&pool)
    .await?
    .ok_or(AppError::NotFound)?;
    
    Ok(Json(EntryResponse::from(entry)))
}

pub async fn update(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateEntryRequest>,
) -> Result<Json<EntryResponse>, AppError> {
    let existing = sqlx::query_as::<_, Entry>(
        "SELECT * FROM entries WHERE id = $1"
    )
    .bind(id)
    .fetch_optional(&pool)
    .await?
    .ok_or(AppError::NotFound)?;
    
    let slug = req.slug.unwrap_or(existing.slug);
    let status = req.status.unwrap_or(existing.status);
    let data = req.data.unwrap_or(existing.data);
    let meta = req.meta.unwrap_or(existing.meta);
    
    let entry = sqlx::query_as::<_, Entry>(
        "UPDATE entries SET slug = $1, status = $2, data = $3, meta = $4 WHERE id = $5 RETURNING *"
    )
    .bind(&slug)
    .bind(&status)
    .bind(&data)
    .bind(&meta)
    .bind(id)
    .fetch_one(&pool)
    .await?;
    
    Ok(Json(EntryResponse::from(entry)))
}

pub async fn delete(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    let result = sqlx::query("DELETE FROM entries WHERE id = $1")
        .bind(id)
        .execute(&pool)
        .await?;
    
    if result.rows_affected() == 0 {
        return Err(AppError::NotFound);
    }
    
    Ok(StatusCode::NO_CONTENT)
}
use axum::{Json, extract::{State, Path}, http::StatusCode};
use sqlx::PgPool;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use serde_json::Value as JsonValue;
use crate::{models::Entry, error::AppError};

#[derive(Debug, Deserialize)]
pub struct CreateEntryRequest {
    pub content_type_id: Uuid,
    pub author_id: Uuid,
    pub slug: String,
    pub data: JsonValue,
    pub meta: Option<JsonValue>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateEntryRequest {
    pub slug: Option<String>,
    pub status: Option<String>,
    pub data: Option<JsonValue>,
    pub meta: Option<JsonValue>,
}

#[derive(Debug, Serialize)]
pub struct EntryResponse {
    pub id: Uuid,
    pub content_type_id: Uuid,
    pub author_id: Uuid,
    pub slug: String,
    pub status: String,
    pub data: JsonValue,
    pub meta: JsonValue,
    pub published_at: Option<chrono::DateTime<chrono::Utc>>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl From<Entry> for EntryResponse {
    fn from(entry: Entry) -> Self {
        EntryResponse {
            id: entry.id,
            content_type_id: entry.content_type_id,
            author_id: entry.author_id,
            slug: entry.slug,
            status: entry.status,
            data: entry.data,
            meta: entry.meta,
            published_at: entry.published_at,
            created_at: entry.created_at,
            updated_at: entry.updated_at,
        }
    }
}

pub async fn create(
    State(pool): State<PgPool>,
    Json(req): Json<CreateEntryRequest>,
) -> Result<(StatusCode, Json<EntryResponse>), AppError> {
    let meta = req.meta.unwrap_or(serde_json::json!({}));
    
    let entry = sqlx::query_as::<_, Entry>(
        "INSERT INTO entries (content_type_id, author_id, slug, data, meta) VALUES ($1, $2, $3, $4, $5) RETURNING *"
    )
    .bind(req.content_type_id)
    .bind(req.author_id)
    .bind(&req.slug)
    .bind(&req.data)
    .bind(&meta)
    .fetch_one(&pool)
    .await?;
    
    Ok((StatusCode::CREATED, Json(EntryResponse::from(entry))))
}

pub async fn list(
    State(pool): State<PgPool>,
) -> Result<Json<Vec<EntryResponse>>, AppError> {
    let entries = sqlx::query_as::<_, Entry>(
        "SELECT * FROM entries ORDER BY created_at DESC"
    )
    .fetch_all(&pool)
    .await?;
    
    Ok(Json(entries.into_iter().map(EntryResponse::from).collect()))
}

pub async fn get(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<Json<EntryResponse>, AppError> {
    let entry = sqlx::query_as::<_, Entry>(
        "SELECT * FROM entries WHERE id = $1"
    )
    .bind(id)
    .fetch_optional(&pool)
    .await?
    .ok_or(AppError::NotFound)?;
    
    Ok(Json(EntryResponse::from(entry)))
}

pub async fn update(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateEntryRequest>,
) -> Result<Json<EntryResponse>, AppError> {
    let existing = sqlx::query_as::<_, Entry>(
        "SELECT * FROM entries WHERE id = $1"
    )
    .bind(id)
    .fetch_optional(&pool)
    .await?
    .ok_or(AppError::NotFound)?;
    
    let slug = req.slug.unwrap_or(existing.slug);
    let status = req.status.unwrap_or(existing.status);
    let data = req.data.unwrap_or(existing.data);
    let meta = req.meta.unwrap_or(existing.meta);
    
    let entry = sqlx::query_as::<_, Entry>(
        "UPDATE entries SET slug = $1, status = $2, data = $3, meta = $4 WHERE id = $5 RETURNING *"
    )
    .bind(&slug)
    .bind(&status)
    .bind(&data)
    .bind(&meta)
    .bind(id)
    .fetch_one(&pool)
    .await?;
    
    Ok(Json(EntryResponse::from(entry)))
}

pub async fn delete(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    let result = sqlx::query("DELETE FROM entries WHERE id = $1")
        .bind(id)
        .execute(&pool)
        .await?;
    
    if result.rows_affected() == 0 {
        return Err(AppError::NotFound);
    }
    
    Ok(StatusCode::NO_CONTENT)
}
