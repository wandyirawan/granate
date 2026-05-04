use axum::{
    extract::{Path, Extension},
    http::StatusCode,
    Json,
};
use sqlx::PgPool;
use serde::Deserialize;
use uuid::Uuid;
use crate::{models::ContentType, error::AppError, middleware::AuthUser};

#[derive(Debug, Deserialize)]
pub struct CreateContentTypeRequest {
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    #[serde(default)]
    pub schema_json: serde_json::Value,
}

pub async fn create(
    Extension(pool): Extension<PgPool>,
    AuthUser(_user_id): AuthUser,
    Json(req): Json<CreateContentTypeRequest>,
) -> Result<Json<ContentType>, AppError> {
    let content_type = sqlx::query_as::<_, ContentType>(
        "INSERT INTO content_types (name, slug, description, schema_json) 
         VALUES ($1, $2, $3, $4) 
         RETURNING *"
    )
    .bind(&req.name)
    .bind(&req.slug)
    .bind(&req.description)
    .bind(&req.schema_json)
    .fetch_one(&pool)
    .await?;
    
    Ok(Json(content_type))
}

pub async fn list(
    Extension(pool): Extension<PgPool>,
) -> Result<Json<Vec<ContentType>>, AppError> {
    let content_types = sqlx::query_as::<_, ContentType>(
        "SELECT * FROM content_types ORDER BY created_at DESC"
    )
    .fetch_all(&pool)
    .await?;
    
    Ok(Json(content_types))
}

pub async fn get(
    Extension(pool): Extension<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<Json<ContentType>, AppError> {
    let content_type = sqlx::query_as::<_, ContentType>(
        "SELECT * FROM content_types WHERE id = $1"
    )
    .bind(id)
    .fetch_optional(&pool)
    .await?
    .ok_or(AppError::NotFound)?;
    
    Ok(Json(content_type))
}

#[derive(Debug, Deserialize)]
pub struct UpdateContentTypeRequest {
    pub name: Option<String>,
    pub slug: Option<String>,
    pub description: Option<String>,
    pub schema_json: Option<serde_json::Value>,
}

pub async fn update(
    Extension(pool): Extension<PgPool>,
    Path(id): Path<Uuid>,
    AuthUser(_user_id): AuthUser,
    Json(req): Json<UpdateContentTypeRequest>,
) -> Result<Json<ContentType>, AppError> {
    let content_type = sqlx::query_as::<_, ContentType>(
        "UPDATE content_types 
         SET name = COALESCE($1, name),
             slug = COALESCE($2, slug),
             description = COALESCE($3, description),
             schema_json = COALESCE($4, schema_json)
         WHERE id = $5
         RETURNING *"
    )
    .bind(req.name)
    .bind(req.slug)
    .bind(req.description)
    .bind(req.schema_json)
    .bind(id)
    .fetch_optional(&pool)
    .await?
    .ok_or(AppError::NotFound)?;
    
    Ok(Json(content_type))
}

pub async fn delete(
    Extension(pool): Extension<PgPool>,
    Path(id): Path<Uuid>,
    AuthUser(_user_id): AuthUser,
) -> Result<StatusCode, AppError> {
    let result = sqlx::query("DELETE FROM content_types WHERE id = $1")
        .bind(id)
        .execute(&pool)
        .await?;
    
    if result.rows_affected() == 0 {
        return Err(AppError::NotFound);
    }
    
    Ok(StatusCode::NO_CONTENT)
}
