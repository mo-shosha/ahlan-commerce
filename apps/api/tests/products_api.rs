use std::sync::Arc;

use api::{config::Config, constants::routes, router, state::AppState};
use axum::{
    Router,
    body::Body,
    http::{Method, Request, StatusCode, header},
};
use catalog::{
    ProductCatalog,
    testing::{FixedClock, FixedIds},
};
use http_body_util::BodyExt;
use serde_json::{Value, json};
use tower::ServiceExt;

const FIRST_ID: &str = "018f7b2a-9f62-7d0c-8c4f-7a1f37c1d001";
const SECOND_ID: &str = "018f7b2a-9f62-7d0c-8c4f-7a1f37c1d002";
const NOW: &str = "2026-01-01T00:00:00Z";

fn test_app() -> Router {
    let config = Config::from_lookup(|_| None).expect("default config is valid");
    let catalog = ProductCatalog::new(
        Arc::new(FixedIds::new([FIRST_ID, SECOND_ID])),
        Arc::new(FixedClock::from_rfc3339(NOW)),
    );
    router(AppState::new(config, catalog))
}

async fn send(app: &Router, request: Request<Body>) -> (StatusCode, Value) {
    let response = app.clone().oneshot(request).await.expect("request failed");
    let status = response.status();
    let bytes = response
        .into_body()
        .collect()
        .await
        .expect("body failed")
        .to_bytes();
    let body = serde_json::from_slice(&bytes).expect("response body is JSON");
    (status, body)
}

fn get(uri: &str) -> Request<Body> {
    Request::get(uri).body(Body::empty()).unwrap()
}

fn post_json(uri: &str, body: Value) -> Request<Body> {
    Request::builder()
        .method(Method::POST)
        .uri(uri)
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(body.to_string()))
        .unwrap()
}

fn laptop_request() -> Value {
    json!({
        "title": "Laptop",
        "handle": "laptop",
        "price_cents": 50000,
        "inventory_quantity": 10,
        "published": true
    })
}

#[tokio::test]
async fn health_returns_ok() {
    let (status, body) = send(&test_app(), get(routes::HEALTH)).await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(body, json!({ "status": "ok" }));
}

#[tokio::test]
async fn list_products_starts_empty() {
    let (status, body) = send(&test_app(), get(routes::PRODUCTS)).await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(body, json!([]));
}

#[tokio::test]
async fn create_product_returns_created_product() {
    let (status, body) = send(&test_app(), post_json(routes::PRODUCTS, laptop_request())).await;

    assert_eq!(status, StatusCode::CREATED);
    assert_eq!(
        body,
        json!({
            "id": FIRST_ID,
            "title": "Laptop",
            "handle": "laptop",
            "price_cents": 50000,
            "inventory_quantity": 10,
            "published": true,
            "created_at": NOW,
            "updated_at": NOW
        })
    );
}

#[tokio::test]
async fn created_products_are_listed_in_creation_order() {
    let app = test_app();
    let phone = json!({
        "title": "Phone",
        "handle": "phone",
        "price_cents": 30000,
        "inventory_quantity": 5,
        "published": false
    });

    send(&app, post_json(routes::PRODUCTS, laptop_request())).await;
    send(&app, post_json(routes::PRODUCTS, phone)).await;
    let (status, body) = send(&app, get(routes::PRODUCTS)).await;

    assert_eq!(status, StatusCode::OK);
    let products = body.as_array().expect("list response is an array");
    assert_eq!(products.len(), 2);
    assert_eq!(products[0]["id"], FIRST_ID);
    assert_eq!(products[0]["handle"], "laptop");
    assert_eq!(products[1]["id"], SECOND_ID);
    assert_eq!(products[1]["handle"], "phone");
}
