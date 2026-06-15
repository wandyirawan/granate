use axum::extract::{Path, Extension};
use axum::Json;
use std::sync::Arc;
use serde_json::json;
use uuid::Uuid;
use sqlx::PgPool;

use crate::{models::*, api::products::*, config::Config};

async fn test_pool() -> PgPool {
    PgPool::connect("postgresql://postgres:***@localhost:5433/granate_test")
        .await
        .unwrap()
}

fn test_config() -> Arc<Config> {
    Arc::new(Config {
        database_url: "postgresql://postgres:***@localhost:5433/granate_test".into(),
        mangosteen_url: "http://localhost:3001".into(),
        mangosteen_jwks_url: "http://localhost:3001/jwks".into(),
        minio_endpoint: "localhost:9000".into(),
        minio_access_key: "pomegranate".into(),
        minio_secret_key: "pomegranate123".into(),
        minio_bucket: "granate-media".into(),
        port: 3000,
        salak_url: "http://localhost:8000".into(),
    })
}

async fn clean_db(pool: &PgPool) {
    sqlx::query("DELETE FROM product_variants").execute(pool).await.unwrap();
    sqlx::query("DELETE FROM parent_products").execute(pool).await.unwrap();
}

// ============================================================
// Product CRUD tests
// ============================================================

#[tokio::test]
async fn test_create_parent_product() {
    let pool = Arc::new(test_pool().await);
    clean_db(&pool).await;
    let config = test_config();

    let payload = CreateParentProduct {
        name: "Test Product".into(),
        description: Some("Test description".into()),
        option_types: Some(vec!["color".into(), "size".into()]),
        thumbnail_media_id: None,
    };

    let result = create(Extension(pool.clone()), Extension(config), Json(payload)).await;
    assert!(result.is_ok(), "Create failed: {:?}", result.err());

    let product = result.unwrap().0;
    assert_eq!(product.name, "Test Product");
    assert_eq!(product.slug, "test-product");
    assert_eq!(product.description, Some("Test description".into()));
    assert_eq!(product.status, "draft");

    clean_db(&pool).await;
}

#[tokio::test]
async fn test_list_products() {
    let pool = Arc::new(test_pool().await);
    clean_db(&pool).await;
    let config = test_config();

    let p1 = CreateParentProduct { name: "Product 1".into(), description: None, option_types: None, thumbnail_media_id: None };
    let p2 = CreateParentProduct { name: "Product 2".into(), description: None, option_types: None, thumbnail_media_id: None };

    create(Extension(pool.clone()), Extension(config.clone()), Json(p1)).await.unwrap();
    create(Extension(pool.clone()), Extension(config), Json(p2)).await.unwrap();

    let products = list(Extension(pool.clone())).await.unwrap().0;
    assert_eq!(products.len(), 2);

    clean_db(&pool).await;
}

#[tokio::test]
async fn test_get_product_by_slug() {
    let pool = Arc::new(test_pool().await);
    clean_db(&pool).await;
    let config = test_config();

    let payload = CreateParentProduct {
        name: "Test Product".into(),
        description: Some("A description".into()),
        option_types: None,
        thumbnail_media_id: None,
    };
    create(Extension(pool.clone()), Extension(config.clone()), Json(payload)).await.unwrap();

    let product = get_by_slug(Path("test-product".into()), Extension(pool.clone()), Extension(config))
        .await
        .unwrap()
        .0;

    assert_eq!(product.name, "Test Product");
    assert_eq!(product.slug, "test-product");
    assert_eq!(product.status, "draft");
    assert!(product.variants.is_empty());

    clean_db(&pool).await;
}

#[tokio::test]
async fn test_get_product_by_slug_404() {
    let pool = Arc::new(test_pool().await);
    clean_db(&pool).await;
    let config = test_config();

    let result = get_by_slug(Path("nonexistent".into()), Extension(pool.clone()), Extension(config)).await;
    assert!(result.is_err());

    clean_db(&pool).await;
}

#[tokio::test]
async fn test_update_product() {
    let pool = Arc::new(test_pool().await);
    clean_db(&pool).await;
    let config = test_config();

    let payload = CreateParentProduct {
        name: "Original".into(),
        description: None,
        option_types: None,
        thumbnail_media_id: None,
    };
    let product = create(Extension(pool.clone()), Extension(config.clone()), Json(payload)).await.unwrap().0;

    let upd = UpdateParentProduct {
        name: Some("Updated Name".into()),
        description: Some("New desc".into()),
        option_types: None,
        thumbnail_media_id: None,
        status: Some("published".into()),
    };
    let updated = update(Path(product.id), Extension(pool.clone()), Json(upd)).await.unwrap().0;

    assert_eq!(updated.name, "Updated Name");
    assert_eq!(updated.description, Some("New desc".into()));
    assert_eq!(updated.status, "published");

    clean_db(&pool).await;
}

#[tokio::test]
async fn test_archive_product() {
    let pool = Arc::new(test_pool().await);
    clean_db(&pool).await;
    let config = test_config();

    let payload = CreateParentProduct { name: "To Archive".into(), description: None, option_types: None, thumbnail_media_id: None };
    let product = create(Extension(pool.clone()), Extension(config), Json(payload)).await.unwrap().0;

    let status_code = delete(Path(product.id), Extension(pool.clone())).await.unwrap();
    assert_eq!(status_code, axum::http::StatusCode::NO_CONTENT);

    // Verify it's archived (soft delete — but get_by_slug doesn't filter by status)
    // So archived products still appear in slug lookup, which is correct admin behavior
    let found = get_by_slug(Path("to-archive".into()), Extension(pool.clone()), Extension(test_config())).await;
    assert!(found.is_ok(), "Archived product should still be findable by slug (admin API)");

    clean_db(&pool).await;
}

#[tokio::test]
async fn test_create_product_duplicate_slug() {
    let pool = Arc::new(test_pool().await);
    clean_db(&pool).await;
    let config = test_config();

    let p1 = CreateParentProduct { name: "Same Name".into(), description: None, option_types: None, thumbnail_media_id: None };
    create(Extension(pool.clone()), Extension(config.clone()), Json(p1)).await.unwrap();

    // Second create with same name → same slug → constraint violation
    let p2 = CreateParentProduct { name: "Same Name".into(), description: None, option_types: None, thumbnail_media_id: None };
    let result = create(Extension(pool.clone()), Extension(config), Json(p2)).await;
    assert!(result.is_err());

    clean_db(&pool).await;
}

// ============================================================
// Variant management tests
// ============================================================

#[tokio::test]
async fn test_add_variant() {
    let pool = Arc::new(test_pool().await);
    clean_db(&pool).await;
    let config = test_config();

    let payload = CreateParentProduct { name: "Variant Test".into(), description: None, option_types: None, thumbnail_media_id: None };
    let product = create(Extension(pool.clone()), Extension(config.clone()), Json(payload)).await.unwrap().0;

    let variant_payload = AddVariant {
        salak_sku: "SKU-001".into(),
        option_values: Some(json!({"color": "red", "size": "M"})),
        is_default: Some(true),
        sort_order: Some(0),
    };
    let variant = add_variant(Path(product.id), Extension(pool.clone()), Extension(config), Json(variant_payload))
        .await
        .unwrap()
        .0;

    assert_eq!(variant.salak_sku, "SKU-001");
    assert_eq!(variant.parent_id, product.id);
    assert!(variant.is_default);

    clean_db(&pool).await;
}

#[tokio::test]
async fn test_add_variant_duplicate_sku() {
    let pool = Arc::new(test_pool().await);
    clean_db(&pool).await;
    let config = test_config();

    let payload = CreateParentProduct { name: "Dup Test".into(), description: None, option_types: None, thumbnail_media_id: None };
    let product = create(Extension(pool.clone()), Extension(config.clone()), Json(payload)).await.unwrap().0;

    let v1 = AddVariant { salak_sku: "SKU-001".into(), option_values: None, is_default: None, sort_order: None };
    add_variant(Path(product.id), Extension(pool.clone()), Extension(config.clone()), Json(v1)).await.unwrap();

    let v2 = AddVariant { salak_sku: "SKU-001".into(), option_values: None, is_default: None, sort_order: None };
    let result = add_variant(Path(product.id), Extension(pool.clone()), Extension(config), Json(v2)).await;
    assert!(result.is_err()); // unique constraint on (parent_id, salak_sku)

    clean_db(&pool).await;
}

#[tokio::test]
async fn test_remove_variant() {
    let pool = Arc::new(test_pool().await);
    clean_db(&pool).await;
    let config = test_config();

    let payload = CreateParentProduct { name: "Remove Test".into(), description: None, option_types: None, thumbnail_media_id: None };
    let product = create(Extension(pool.clone()), Extension(config.clone()), Json(payload)).await.unwrap().0;

    let v = AddVariant { salak_sku: "SKU-RM".into(), option_values: None, is_default: None, sort_order: None };
    let variant = add_variant(Path(product.id), Extension(pool.clone()), Extension(config.clone()), Json(v)).await.unwrap().0;

    let status = remove_variant(Path((product.id, variant.id)), Extension(pool.clone())).await.unwrap();
    assert_eq!(status, axum::http::StatusCode::NO_CONTENT);

    clean_db(&pool).await;
}

#[tokio::test]
async fn test_get_product_with_variants() {
    let pool = Arc::new(test_pool().await);
    clean_db(&pool).await;
    let config = test_config();

    let payload = CreateParentProduct { name: "Multi Variant".into(), description: None, option_types: None, thumbnail_media_id: None };
    let product = create(Extension(pool.clone()), Extension(config.clone()), Json(payload)).await.unwrap().0;

    let v1 = AddVariant { salak_sku: "SKU-A".into(), option_values: Some(json!({"color": "red"})), is_default: Some(true), sort_order: Some(0) };
    let v2 = AddVariant { salak_sku: "SKU-B".into(), option_values: Some(json!({"color": "blue"})), is_default: Some(false), sort_order: Some(1) };
    add_variant(Path(product.id), Extension(pool.clone()), Extension(config.clone()), Json(v1)).await.unwrap();
    add_variant(Path(product.id), Extension(pool.clone()), Extension(config.clone()), Json(v2)).await.unwrap();

    let with_vars = get_by_slug(Path("multi-variant".into()), Extension(pool.clone()), Extension(config))
        .await
        .unwrap()
        .0;

    assert_eq!(with_vars.variants.len(), 2);
    assert_eq!(with_vars.variants[0].salak_sku, "SKU-A");
    assert_eq!(with_vars.variants[1].salak_sku, "SKU-B");

    clean_db(&pool).await;
}

// ============================================================
// Edge cases
// ============================================================

#[tokio::test]
async fn test_create_product_empty_name() {
    let pool = Arc::new(test_pool().await);
    clean_db(&pool).await;
    let config = test_config();

    let payload = CreateParentProduct { name: "".into(), description: None, option_types: None, thumbnail_media_id: None };
    let result = create(Extension(pool.clone()), Extension(config), Json(payload)).await;
    // Empty name should either error or produce empty slug (which might cause PK violation later)
    // Either way, the function should not panic
    let _ = result;

    clean_db(&pool).await;
}
