pub mod config;
pub mod constants;
pub mod db;
pub mod dto;
pub mod handlers;
pub mod state;
pub mod app_error;
pub mod middleware;
use axum::{Router, http::Request,http::Response, middleware::from_fn, routing::get,extract::MatchedPath,body::Body,
};
use crate::constants::routes;
use crate::state::AppState;
use crate::middleware::request_id_middleware;

use tower_http::trace::{TraceLayer};
use tracing::{field, info_span, Span};


pub fn router(state: AppState) -> Router {

    let trace_layer = TraceLayer::new_for_http()
        .make_span_with(|request: &Request<Body>| {
            let request_id = request
                .extensions()
                .get::<String>()
                .cloned()
                .unwrap_or_else(|| "missing".to_string());

            let route = request
                .extensions()
                .get::<MatchedPath>()
                .map(|path| path.as_str().to_owned())
                .unwrap_or_else(|| request.uri().path().to_owned());

            info_span!(
                "http_request",
                request_id = %request_id,
                method = %request.method(),
                route = %route,
                status = field::Empty,
                latency_ms = field::Empty,
                error_code = field::Empty,
            )
        })
        .on_response(
            |response: &Response<axum::body::Body>,
             latency: std::time::Duration,
             span: &Span| {
                let status = response.status().as_u16();
                let latency_ms = latency.as_millis() as u64;

                span.record("status", status);
                span.record("latency_ms", latency_ms);

                tracing::info!(
                    parent: span,
                    status,
                    latency_ms,
                    "HTTP request completed"
                );
            },
        );


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
        .fallback(handlers::not_found)
        .layer(trace_layer)
        .layer(from_fn(request_id_middleware))
        .with_state(state)
}



