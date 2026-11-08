use axum::{Json, extract::State, http::StatusCode};

use crate::dto::{CreateProductRequest, HealthResponse, ProductResponse};
use crate::state::AppState;

pub async fn root() -> &'static str {
    "Welcome to Ahlan Commerce by Shosha"
}

pub async fn health() -> Json<HealthResponse> {
    Json(HealthResponse { status: "ok" })
}

pub async fn list_products(State(state): State<AppState>) -> Json<Vec<ProductResponse>> {
    let products = state
        .catalog
        .lock()
        .expect("catalog lock poisoned")
        .list_products();

    Json(products.into_iter().map(ProductResponse::from).collect())
}

pub async fn create_product(
    State(state): State<AppState>,
    Json(request): Json<CreateProductRequest>,
) -> (StatusCode, Json<ProductResponse>) {
    let product = state
        .catalog
        .lock()
        .expect("catalog lock poisoned")
        .create_product(request.into());

    (StatusCode::CREATED, Json(product.into()))
}
