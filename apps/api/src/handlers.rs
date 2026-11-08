use axum::{Json, extract::State, http::StatusCode,Extension};
use rootcause::{Report, report, prelude::ResultExt};

use crate::dto::{CreateProductRequest, HealthResponse, ProductResponse};
use crate::state::AppState;
use crate::app_error::AppError;

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
    Extension(request_id): Extension<String>,
    Json(request): Json<CreateProductRequest>,
) -> Result<(StatusCode, Json<ProductResponse>), AppError> {
    let mut catalog = state.catalog.lock().map_err(|_| {
        AppError::internal(
            report!("catalog lock was poisoned by an earlier panic"),
            request_id.clone(),
        )
    })?;

    let product = catalog
        .create_product(request.into())
        .map_err(|error| {
            AppError::from_catalog(error, request_id.clone())
        })?;

    Ok((StatusCode::CREATED, Json(product.into())))
}



pub fn simulate_inventory_sync() -> Result<u32, Report> {
    let stored_quantity = "ten";

    let quantity = stored_quantity
        .parse::<u32>()
        .context("Inventory sync failed: stored quantity is not a number")?;

    Ok(quantity)
}

pub async fn simulated_internal_failure(Extension(request_id): Extension<String>,) -> Result<StatusCode, AppError> {
    simulate_inventory_sync()
        .map_err(|cause| AppError::internal(cause, request_id))?;

    Ok(StatusCode::NO_CONTENT)
}


