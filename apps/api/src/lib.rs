pub mod config;
pub mod constants;
pub mod dto;
pub mod handlers;
pub mod state;
pub mod app_error;
pub mod middleware;
use axum::{Router, routing::get,middleware::from_fn};
use crate::constants::routes;
use crate::state::AppState;
use crate::middleware::request_id_middleware;


pub fn router(state: AppState) -> Router {
    Router::new()
        .route(routes::ROOT, get(handlers::root))
        .route(routes::HEALTH, get(handlers::health))
        .route(
            routes::PRODUCTS,
            get(handlers::list_products).post(handlers::create_product),
        )
        .route(
            routes::SIMULATED_INTERNAL_FAILURE,
            get(handlers::simulated_internal_failure),
        )
        .layer(from_fn(request_id_middleware))
        .with_state(state)
}



