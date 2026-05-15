use axum::{
    extract::Extension,
    response::Json,
};
use aws_sdk_s3::Client as S3Client;
use aws_sdk_s3::config::{Credentials, Region};
use axum_extra::extract::Multipart;
use serde_json::{json, Value};
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

use crate::config::Config;
use crate::models::Media;
use crate::media::{process_image, upload_to_minio, build_s3_url};

pub async fn upload(
    Extension(pool): Extension<PgPool>,
    Extension(config): Extension<Arc<Config>>,
    mut multipart: Multipart,
) -> Result<Json<Value>, crate::error::AppError> {
    while let Some(field) = multipart.next_field().await
        .map_err(|e| crate::error::AppError::BadRequest(format!("Multipart: {e}")))?
    {
        let filename = field
            .file_name()
            .unwrap_or("unknown.jpg")
            .to_string();
        let content_type = field
            .content_type()
            .map(|c| c.to_string())
            .unwrap_or_else(|| "application/octet-stream".into());
        let data = field.bytes().await
            .map_err(|e| crate::error::AppError::BadRequest(format!("Read: {e}")))?;

        if !content_type.starts_with("image/") {
            return Err(crate::error::AppError::BadRequest(
                "Only image files are accepted".into(),
            ));
        }

        if data.len() > 10 * 1024 * 1024 {
            return Err(crate::error::AppError::BadRequest("File too large (max 10MB)".into()));
        }

        let processed = process_image(&data, &filename)
            .map_err(|e| crate::error::AppError::BadRequest(e))?;

        let media_id = Uuid::new_v4();
        let ext = filename.split('.').last().unwrap_or("jpg");
        let short_id = &media_id.to_string()[..8];

        // S3 client
        let s3_config = aws_sdk_s3::Config::builder()
            .credentials_provider(Credentials::new(
                &config.minio_access_key,
                &config.minio_secret_key,
                None,
                None,
                "minio",
            ))
            .region(Region::new("us-east-1"))
            .endpoint_url(format!("http://{}", config.minio_endpoint))
            .force_path_style(true)
            .build();

        let client = S3Client::from_conf(s3_config);

        let _variants = upload_to_minio(&processed, media_id, &filename, &client, &config.minio_bucket)
            .await
            .map_err(|e| crate::error::AppError::Internal(e))?;

        // Save media record
        let mut tx = pool.begin().await
            .map_err(|e| crate::error::AppError::Internal(format!("DB: {e}")))?;

        let storage_key = format!("media/{}/{}/original.{}", short_id, media_id, ext);

        let media = sqlx::query_as::<_, Media>(
            "INSERT INTO media (id, filename, storage_key, mime_type, size_bytes, width, height, uploaded_by, metadata)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
             RETURNING *"
        )
        .bind(media_id)
        .bind(&filename)
        .bind(&storage_key)
        .bind(&processed.original.mime_type)
        .bind(processed.original.size_bytes as i64)
        .bind(processed.original.width as i32)
        .bind(processed.original.height as i32)
        .bind(Uuid::nil())
        .bind(json!({ "variants": processed.variants.iter().map(|v| v.name.clone()).collect::<Vec<_>>() }))
        .fetch_one(&mut *tx)
        .await
        .map_err(|e| crate::error::AppError::Internal(format!("DB: {e}")))?;

        // Save variant records
        for v in &processed.variants {
            let vkey = format!("media/{}/{}/{}.{}", short_id, media_id, &v.name, ext);
            sqlx::query(
                "INSERT INTO media_variants (media_id, variant, storage_key, width, height, size_bytes)
                 VALUES ($1, $2, $3, $4, $5, $6)"
            )
            .bind(media_id)
            .bind(&v.name)
            .bind(&vkey)
            .bind(v.width as i32)
            .bind(v.height as i32)
            .bind(v.size_bytes as i64)
            .execute(&mut *tx)
            .await
            .map_err(|e| crate::error::AppError::Internal(format!("DB: {e}")))?;
        }

        tx.commit().await
            .map_err(|e| crate::error::AppError::Internal(format!("DB: {e}")))?;

        let url = build_s3_url(&config.minio_endpoint, &config.minio_bucket, &storage_key);
        let variant_urls: Vec<Value> = processed.variants.iter().map(|v| {
            let vkey = format!("media/{}/{}/{}.{}", short_id, media_id, &v.name, ext);
            json!({
                "variant": v.name,
                "width": v.width,
                "height": v.height,
                "url": build_s3_url(&config.minio_endpoint, &config.minio_bucket, &vkey)
            })
        }).collect();

        return Ok(Json(json!({
            "id": media.id,
            "filename": media.filename,
            "url": url,
            "width": media.width,
            "height": media.height,
            "size_bytes": media.size_bytes,
            "mime_type": media.mime_type,
            "variants": variant_urls,
            "created_at": media.created_at,
        })));
    }

    Err(crate::error::AppError::BadRequest("No file provided".into()))
}

pub async fn list_media(
    Extension(pool): Extension<PgPool>,
    Extension(config): Extension<Arc<Config>>,
) -> Result<Json<Vec<Value>>, crate::error::AppError> {
    let rows = sqlx::query_as::<_, Media>("SELECT * FROM media ORDER BY created_at DESC LIMIT 50")
        .fetch_all(&pool)
        .await
        .map_err(|e| crate::error::AppError::Internal(format!("DB: {e}")))?;

    let result: Vec<Value> = rows.iter().map(|m| {
        json!({
            "id": m.id,
            "filename": m.filename,
            "url": build_s3_url(&config.minio_endpoint, &config.minio_bucket, &m.storage_key),
            "width": m.width,
            "height": m.height,
            "size_bytes": m.size_bytes,
            "mime_type": m.mime_type,
            "created_at": m.created_at,
        })
    }).collect();

    Ok(Json(result))
}
