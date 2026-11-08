use catalog::{NewProduct, Product, Timestamp};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize)]
pub struct HealthResponse {
    pub status: &'static str,
}

#[derive(Debug, Deserialize)]
pub struct CreateProductRequest {
    pub title: String,
    pub handle: String,
    pub price_cents: u32,
    pub inventory_quantity: u32,
    pub published: bool,
}

impl From<CreateProductRequest> for NewProduct {
    fn from(request: CreateProductRequest) -> Self {
        Self {
            title: request.title,
            handle: request.handle,
            price_cents: request.price_cents,
            inventory_quantity: request.inventory_quantity,
            published: request.published,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct ProductResponse {
    pub id: Uuid,
    pub title: String,
    pub handle: String,
    pub price_cents: u32,
    pub inventory_quantity: u32,
    pub published: bool,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
}

impl From<Product> for ProductResponse {
    fn from(product: Product) -> Self {
        Self {
            id: product.id.as_uuid(),
            title: product.title,
            handle: product.handle,
            price_cents: product.price_cents,
            inventory_quantity: product.inventory_quantity,
            published: product.published,
            created_at: product.created_at,
            updated_at: product.updated_at,
        }
    }
}
