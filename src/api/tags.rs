use axum::{
    extract::{Path, Extension},
    http::StatusCode,
    Json,
};
use sqlx::PgPool;
use serde::Deserialize;
use uuid::Uuid;
use crate::{models::Tag, error::AppError, middleware::AuthUser};

#[derive(Debug, Deserialize)]
pub struct CreateTagRequest {
    pub name: String,
    pub slug: String,
}

pub async fn create(
    Extension(pool): Extension<PgPool>,
    AuthUser(_user_id): AuthUser,
    Json(req): Json<CreateTagRequest>,
) -> Result<Json<Tag>, AppError> {
    let tag = sqlx::query_as::<_, Tag>(
        "INSERT INTO tags (name, slug) VALUES ($1, $2) RETURNING *"
    )
    .bind(&req.name)
    .bind(&req.slug)
    .fetch_one(&pool)
    .await?;
    
    Ok(Json(tag))
}

pub async fn list(
    Extension(pool): Extension<PgPool>,
) -> Result<Json<Vec<Tag>>, AppError> {
    let tags = sqlx::query_as::<_, Tag>(
        "SELECT * FROM tags ORDER BY name"
    )
    .fetch_all(&pool)
    .await?;
    
    Ok(Json(tags))
}

pub async fn delete(
    Extension(pool): Extension<PgPool>,
    Path(id): Path<Uuid>,
    AuthUser(_user_id): AuthUser,
) -> Result<StatusCode, AppError> {
    let result = sqlx::query("DELETE FROM tags WHERE id = $1")
        .bind(id)
        .execute(&pool)
        .await?;
    
    if result.rows_affected() == 0 {
        return Err(AppError::NotFound);
    }
    
    Ok(StatusCode::NO_CONTENT)
}
