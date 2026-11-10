use axum::{Json, extract::State, http::StatusCode,Extension};
use rootcause::{Report, prelude::ResultExt};

use crate::dto::{CreateProductRequest, HealthResponse, ProductResponse};
use crate::state::AppState;
use crate::app_error::{AppError, AppErrorKind};

pub async fn root() -> &'static str {
    "Welcome to Ahlan Commerce by Shosha"
}

pub async fn health() -> Json<HealthResponse> {
    Json(HealthResponse { status: "ok" })
}

pub async fn list_products(
    State(state): State<AppState>,
    Extension(request_id): Extension<String>,
) -> Result<Json<Vec<ProductResponse>>, AppError> {
    let products = state
        .catalog
        .list_products(&state.db)
        .await
        .map_err(|error| AppError::from_catalog(error, request_id))?;

    Ok(Json(products.into_iter().map(ProductResponse::from).collect()))
}



pub async fn create_product(
    State(state): State<AppState>,
    Extension(request_id): Extension<String>,
    Json(request): Json<CreateProductRequest>,
) -> Result<(StatusCode, Json<ProductResponse>), AppError> {
    let product_handle = request.handle.clone();

    let product = state
        .catalog
        .create_product(&state.db, request.into())
        .await
        .map_err(|error| {
            let app_error = AppError::from_catalog(error, request_id.clone());
            tracing::warn!(
                operation = "create_product",
                product_handle = %product_handle,
                error_code = app_error.error_code(),
                "Product creation rejected"
            );
            app_error
        })?;

    tracing::info!(
        operation = "create_product",
        product_id = %product.id,
        product_handle = %product.handle,
        "Product created"
    );

    Ok((StatusCode::CREATED, Json(product.into())))
}

pub async fn not_found(Extension(request_id): Extension<String>) -> AppError {
    AppError::new(AppErrorKind::NotFound, request_id)
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


