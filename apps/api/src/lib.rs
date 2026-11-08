pub mod config;
pub mod constants;
pub mod dto;
pub mod handlers;
pub mod state;

use axum::{Router, routing::get};

use crate::constants::routes;
use crate::state::AppState;

pub fn router(state: AppState) -> Router {
    Router::new()
        .route(routes::ROOT, get(handlers::root))
        .route(routes::HEALTH, get(handlers::health))
        .route(
            routes::PRODUCTS,
            get(handlers::list_products).post(handlers::create_product),
        )
        .with_state(state)
}
