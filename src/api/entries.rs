use axum::{
    extract::{Path, Extension, Query},
    http::StatusCode,
    Json,
};
use sqlx::PgPool;
use serde::Deserialize;
use uuid::Uuid;
use crate::{models::Entry, error::AppError, middleware::AuthenticatedUser};

#[derive(Debug, Deserialize)]
pub struct CreateEntryRequest {
    pub content_type_id: Uuid,
    pub slug: String,
    pub data: serde_json::Value,
    #[serde(default)]
    pub meta: serde_json::Value,
    #[serde(default = "default_status")]
    pub status: String,
}

fn default_status() -> String {
    "draft".to_string()
}

#[derive(Debug, Deserialize, Default)]
pub struct ListEntriesQuery {
    pub content_type_id: Option<Uuid>,
    pub status: Option<String>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

pub async fn create(
    Extension(pool): Extension<PgPool>,
    AuthenticatedUser(claims): AuthenticatedUser,
    Json(req): Json<CreateEntryRequest>,
) -> Result<Json<Entry>, AppError> {
    let author_id = Uuid::parse_str(&claims.sub)
        .map_err(|_| AppError::Validation("Invalid user ID".to_string()))?;
    
    let entry = sqlx::query_as::<_, Entry>(
        "INSERT INTO entries (content_type_id, author_id, slug, status, data, meta) 
         VALUES ($1, $2, $3, $4, $5, $6) 
         RETURNING *"
    )
    .bind(req.content_type_id)
    .bind(author_id)
    .bind(&req.slug)
    .bind(&req.status)
    .bind(&req.data)
    .bind(&req.meta)
    .fetch_one(&pool)
    .await?;
    
    Ok(Json(entry))
}

pub async fn list(
    Extension(pool): Extension<PgPool>,
    Query(query): Query<ListEntriesQuery>,
) -> Result<Json<Vec<Entry>>, AppError> {
    let limit = query.limit.unwrap_or(20).min(100);
    let offset = query.offset.unwrap_or(0);
    
    let entries = if let Some(content_type_id) = query.content_type_id {
        if let Some(status) = query.status {
            sqlx::query_as::<_, Entry>(
                "SELECT * FROM entries WHERE content_type_id = $1 AND status = $2 
                 ORDER BY created_at DESC LIMIT $3 OFFSET $4"
            )
            .bind(content_type_id)
            .bind(status)
            .bind(limit)
            .bind(offset)
            .fetch_all(&pool)
            .await?
        } else {
            sqlx::query_as::<_, Entry>(
                "SELECT * FROM entries WHERE content_type_id = $1 
                 ORDER BY created_at DESC LIMIT $2 OFFSET $3"
            )
            .bind(content_type_id)
            .bind(limit)
            .bind(offset)
            .fetch_all(&pool)
            .await?
        }
    } else {
        sqlx::query_as::<_, Entry>(
            "SELECT * FROM entries ORDER BY created_at DESC LIMIT $1 OFFSET $2"
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&pool)
        .await?
    };
    
    Ok(Json(entries))
}

pub async fn get(
    Extension(pool): Extension<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<Json<Entry>, AppError> {
    let entry = sqlx::query_as::<_, Entry>("SELECT * FROM entries WHERE id = $1")
        .bind(id)
        .fetch_optional(&pool)
        .await?
        .ok_or(AppError::NotFound)?;
    
    Ok(Json(entry))
}

#[derive(Debug, Deserialize)]
pub struct UpdateEntryRequest {
    pub slug: Option<String>,
    pub data: Option<serde_json::Value>,
    pub meta: Option<serde_json::Value>,
    pub status: Option<String>,
    pub published_at: Option<chrono::DateTime<chrono::Utc>>,
}

pub async fn update(
    Extension(pool): Extension<PgPool>,
    Path(id): Path<Uuid>,
    AuthenticatedUser(_claims): AuthenticatedUser,
    Json(req): Json<UpdateEntryRequest>,
) -> Result<Json<Entry>, AppError> {
    let entry = sqlx::query_as::<_, Entry>(
        "UPDATE entries 
         SET slug = COALESCE($1, slug),
             data = COALESCE($2, data),
             meta = COALESCE($3, meta),
             status = COALESCE($4, status),
             published_at = COALESCE($5, published_at)
         WHERE id = $6
         RETURNING *"
    )
    .bind(req.slug)
    .bind(req.data)
    .bind(req.meta)
    .bind(req.status)
    .bind(req.published_at)
    .bind(id)
    .fetch_optional(&pool)
    .await?
    .ok_or(AppError::NotFound)?;
    
    Ok(Json(entry))
}

pub async fn delete(
    Extension(pool): Extension<PgPool>,
    Path(id): Path<Uuid>,
    AuthenticatedUser(_claims): AuthenticatedUser,
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
