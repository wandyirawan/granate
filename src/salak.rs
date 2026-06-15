use reqwest;
use serde::{Deserialize, Serialize};
use crate::error::AppError;

#[derive(Debug, Serialize, Deserialize)]
pub struct SalakProduct {
    pub id: i32,
    pub name: String,
    pub sku: String,
    pub price: Option<f64>,
    pub stock: Option<i32>,
}

pub async fn list_products(base_url: &str) -> Result<Vec<SalakProduct>, AppError> {
    let url = format!("{}/products", base_url.trim_end_matches('/'));
    let client = reqwest::Client::new();
    
    let response = client
        .get(&url)
        .send()
        .await
        .map_err(|e| AppError::SalakApiError(e.to_string()))?;
    
    if response.status().is_success() {
        let products: Vec<SalakProduct> = response
            .json()
            .await
            .map_err(|e| AppError::SalakApiError(e.to_string()))?;
        Ok(products)
    } else {
        Err(AppError::SalakApiError(format!(
            "Failed to fetch products: {}",
            response.status()
        )))
    }
}

pub async fn get_product_cogs(base_url: &str, product_id: i32) -> Result<f64, AppError> {
    let url = format!("{}/products/{}/cogs", base_url.trim_end_matches('/'), product_id);
    let client = reqwest::Client::new();
    
    let response = client
        .get(&url)
        .send()
        .await
        .map_err(|e| AppError::SalakApiError(e.to_string()))?;
    
    if response.status().is_success() {
        let cogs: f64 = response
            .json()
            .await
            .map_err(|e| AppError::SalakApiError(e.to_string()))?;
        Ok(cogs)
    } else {
        Err(AppError::SalakApiError(format!(
            "Failed to fetch COGS for product {}: {}",
            product_id,
            response.status()
        )))
    }
}