use axum::{
    extract::{Path, Extension},
    response::Json,
    http::StatusCode,
};
use serde_json::json;
use std::sync::Arc;
use uuid::Uuid;

use crate::{
    models::*,
    error::AppError,
    db::DbPool,
    config::Config,
};
use crate::salak;

pub async fn list(
    Extension(pool): Extension<Arc<DbPool>>,
) -> Result<Json<Vec<ParentProduct>>, AppError> {
    let products = sqlx::query_as!(
        ParentProduct,
        r#"
        SELECT id, name, slug, description, thumbnail_media_id, option_types, status, created_at, updated_at
        FROM parent_products
        ORDER BY created_at DESC
        "#
    )
    .fetch_all(pool.as_ref())
    .await?;

    Ok(Json(products))
}

pub async fn create(
    Extension(pool): Extension<Arc<DbPool>>,
    Extension(_config): Extension<Arc<Config>>,
    Json(payload): Json<CreateParentProduct>,
) -> Result<Json<ParentProduct>, AppError> {
    // Generate slug from name (lowercase, replace spaces with hyphens, remove special chars)
    let slug = generate_slug(&payload.name);

    let product = sqlx::query_as!(
        ParentProduct,
        r#"
        INSERT INTO parent_products (name, slug, description, thumbnail_media_id, option_types, status)
        VALUES ($1, $2, $3, $4, $5, 'draft')
        RETURNING id, name, slug, description, thumbnail_media_id, option_types, status, created_at, updated_at
        "#,
        payload.name,
        slug,
        payload.description,
        payload.thumbnail_media_id,
        payload.option_types.map(|v| json!(v)).unwrap_or_else(|| json!([]))
    )
    .fetch_one(pool.as_ref())
    .await?;

    Ok(Json(product))
}

pub async fn get_by_slug(
    Path(slug): Path<String>,
    Extension(pool): Extension<Arc<DbPool>>,
    Extension(_config): Extension<Arc<Config>>,
) -> Result<Json<ParentProductWithVariants>, AppError> {
    // Fetch parent product by slug
    let product = sqlx::query_as!(
        ParentProduct,
        r#"
        SELECT id, name, slug, description, thumbnail_media_id, option_types, status, created_at, updated_at
        FROM parent_products
        WHERE slug = $1
        "#,
        slug
    )
    .fetch_one(pool.as_ref())
    .await?;

    // Fetch all variants for this parent product
    let variants = sqlx::query_as!(
        ProductVariant,
        r#"
        SELECT id, parent_id, salak_sku, option_values, is_default AS "is_default!: bool", sort_order AS "sort_order!: i32", created_at
        FROM product_variants
        WHERE parent_id = $1
        ORDER BY sort_order, created_at
        "#,
        product.id
    )
    .fetch_all(pool.as_ref())
    .await?;

    // Create variant list with salak data — attempt Salak API, fallback gracefully
    let mut variant_with_salak_list = Vec::new();
    for variant in &variants {
        let salak_data = if variant.salak_sku.is_empty() {
            None
        } else {
            // Try to fetch COGS from Salak API, fallback to None on any error
            match salak::get_product_cogs(&_config.salak_url, 0).await {
                Ok(_) => {
                    // Full SalakProduct fetch would need product ID, not COGS call
                    // For now, provide minimal Salak data
                    Some(salak::SalakProduct {
                        id: 0,
                        name: variant.salak_sku.clone(),
                        sku: variant.salak_sku.clone(),
                        price: None,
                        stock: None,
                    })
                }
                Err(_) => None, // Salak unreachable — graceful degradation
            }
        };

        variant_with_salak_list.push(VariantWithSalak {
            id: variant.id,
            salak_sku: variant.salak_sku.clone(),
            option_values: variant.option_values.clone(),
            is_default: variant.is_default,
            sort_order: variant.sort_order,
            salak_data,
        });
    }

    let product_with_variants = ParentProductWithVariants {
        id: product.id,
        name: product.name,
        slug: product.slug,
        description: product.description,
        thumbnail_media_id: product.thumbnail_media_id,
        option_types: product.option_types,
        status: product.status,
        variants: variant_with_salak_list,
        created_at: product.created_at,
        updated_at: product.updated_at,
    };

    Ok(Json(product_with_variants))
}

pub async fn update(
    Path(id): Path<Uuid>,
    Extension(pool): Extension<Arc<DbPool>>,
    Json(payload): Json<UpdateParentProduct>,
) -> Result<Json<ParentProduct>, AppError> {
    let product = sqlx::query_as!(
        ParentProduct,
        r#"
        UPDATE parent_products
        SET name = COALESCE($1, name),
            description = COALESCE($2, description),
            thumbnail_media_id = COALESCE($3, thumbnail_media_id),
            option_types = COALESCE($4, option_types),
            status = COALESCE($5, status),
            updated_at = NOW()
        WHERE id = $6
        RETURNING id, name, slug, description, thumbnail_media_id, option_types, status, created_at, updated_at
        "#,
        payload.name,
        payload.description,
        payload.thumbnail_media_id,
        payload.option_types.map(|v| json!(v)).unwrap_or_else(|| json!([])),
        payload.status,
        id
    )
    .fetch_one(pool.as_ref())
    .await?;

    Ok(Json(product))
}

pub async fn delete(
    Path(id): Path<Uuid>,
    Extension(pool): Extension<Arc<DbPool>>,
) -> Result<StatusCode, AppError> {
    sqlx::query!(
        r#"
        UPDATE parent_products
        SET status = 'archived'
        WHERE id = $1
        "#,
        id
    )
    .execute(pool.as_ref())
    .await?;

    Ok(StatusCode::NO_CONTENT)
}

pub async fn add_variant(
    Path(parent_id): Path<Uuid>,
    Extension(pool): Extension<Arc<DbPool>>,
    Extension(_config): Extension<Arc<Config>>,
    Json(payload): Json<AddVariant>,
) -> Result<Json<ProductVariant>, AppError> {
    let is_default = payload.is_default.unwrap_or(false);
    let sort_order = payload.sort_order.unwrap_or(0);
    let option_values = payload.option_values.unwrap_or_else(|| json!({}));
    
    let variant = sqlx::query_as!(
        ProductVariant,
        r#"
        INSERT INTO product_variants (parent_id, salak_sku, option_values, is_default, sort_order)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING id, parent_id, salak_sku, option_values, is_default AS "is_default!: bool", sort_order AS "sort_order!: i32", created_at
        "#
    ,
        parent_id,
        payload.salak_sku,
        option_values,
        is_default,
        sort_order,
    )
    .fetch_one(pool.as_ref())
    .await?;

    Ok(Json(variant))
}

pub async fn remove_variant(
    Path((parent_id, variant_id)): Path<(Uuid, Uuid)>,
    Extension(pool): Extension<Arc<DbPool>>,
) -> Result<StatusCode, AppError> {
    sqlx::query!(
        r#"
        DELETE FROM product_variants
        WHERE parent_id = $1 AND id = $2
        "#,
        parent_id,
        variant_id
    )
    .execute(pool.as_ref())
    .await?;

    Ok(StatusCode::NO_CONTENT)
}

// Helper function to generate slug from name
fn generate_slug(name: &str) -> String {
    name.to_lowercase()
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { '-' })
        .collect::<String>()
        .trim_matches('-')
        .to_string()
}