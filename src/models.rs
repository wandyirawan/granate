use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use sqlx::FromRow;
use serde_json::Value as JsonValue;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct ContentType {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub schema_json: JsonValue,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Entry {
    pub id: Uuid,
    pub content_type_id: Uuid,
    pub author_id: Uuid,
    pub slug: String,
    pub status: String,
    pub data: JsonValue,
    pub meta: JsonValue,
    pub published_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Tag {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Media {
    pub id: Uuid,
    pub filename: String,
    pub storage_key: String,
    pub mime_type: String,
    pub size_bytes: i64,
    pub width: Option<i32>,
    pub height: Option<i32>,
    pub uploaded_by: Uuid,
    pub metadata: JsonValue,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[allow(dead_code)]
pub struct MediaVariant {
    pub id: Uuid,
    pub media_id: Uuid,
    pub variant: String,
    pub storage_key: String,
    pub width: i32,
    pub height: i32,
    pub size_bytes: i64,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[allow(dead_code)]
pub struct ProductPage {
    pub id: Uuid,
    pub salak_product_id: i32,
    pub long_description: Option<String>,
    pub gallery_media_ids: Option<Vec<Uuid>>,
    pub specs: JsonValue,
    pub seo_title: Option<String>,
    pub seo_description: Option<String>,
    pub seo_keywords: Option<String>,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
