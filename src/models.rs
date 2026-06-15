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

// --- Product Variant Models ---
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct ParentProduct {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub thumbnail_media_id: Option<Uuid>,
    pub option_types: JsonValue,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct ProductVariant {
    pub id: Uuid,
    pub parent_id: Uuid,
    pub salak_sku: String,
    pub option_values: JsonValue,
    pub is_default: bool,
    pub sort_order: i32,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct ParentProductWithVariants {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub thumbnail_media_id: Option<Uuid>,
    pub option_types: JsonValue,
    pub status: String,
    pub variants: Vec<VariantWithSalak>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct VariantWithSalak {
    pub id: Uuid,
    pub salak_sku: String,
    pub option_values: JsonValue,
    pub is_default: bool,
    pub sort_order: i32,
    pub salak_data: Option<crate::salak::SalakProduct>,
}

#[derive(Debug, Deserialize)]
pub struct CreateParentProduct {
    pub name: String,
    pub description: Option<String>,
    pub option_types: Option<Vec<String>>,
    pub thumbnail_media_id: Option<Uuid>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateParentProduct {
    pub name: Option<String>,
    pub description: Option<String>,
    pub option_types: Option<Vec<String>>,
    pub thumbnail_media_id: Option<Uuid>,
    pub status: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct AddVariant {
    pub salak_sku: String,
    pub option_values: Option<JsonValue>,
    pub is_default: Option<bool>,
    pub sort_order: Option<i32>,
}
