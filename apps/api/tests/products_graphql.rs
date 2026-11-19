use std::sync::Arc;

use api::{config::Config, constants::{env_keys, routes}, router, state::AppState};
use axum::{
    Router,
    body::Body,
    http::{Method, Request, StatusCode, header},
};
use catalog::{
    ImportJobService, ProductCatalog,
    testing::{FixedClock, FixedIds},
};
use db::testing::TestDb;
use http_body_util::BodyExt;
use serde_json::{Value, json};
use tower::ServiceExt;

const FIRST_ID: &str = "018f7b2a-9f62-7d0c-8c4f-7a1f37c1d001";
const SECOND_ID: &str = "018f7b2a-9f62-7d0c-8c4f-7a1f37c1d002";
const NOW: &str = "2026-01-01T00:00:00Z";

const PRODUCTS_QUERY: &str = r#"
    query {
        products {
            id
            title
            handle
            description
            priceCents
            inventoryQuantity
            published
            publishedAt
            createdAt
            updatedAt
        }
    }
"#;

const CREATE_MUTATION: &str = r#"
    mutation ProductCreate($input: ProductCreateInput!) {
        productCreate(input: $input) {
            id
            title
            handle
            description
            priceCents
            inventoryQuantity
            published
            publishedAt
            createdAt
            updatedAt
        }
    }
"#;

async fn app_for(test_db: &TestDb, ids: &[&str]) -> Router {
    let config = Config::from_lookup(|key| {
        (key == env_keys::DATABASE_URL).then(|| test_db.database_url().to_string())
    })
    .expect("test config is valid");
    let ids = Arc::new(FixedIds::new(ids.iter().copied()));
    let clock = Arc::new(FixedClock::from_rfc3339(NOW));
    let catalog = ProductCatalog::new(ids.clone(), clock.clone());
    let import_jobs = ImportJobService::new(ids, clock);
    router(AppState::new(config, test_db.pool().await, catalog, import_jobs))
}

async fn test_app() -> (Router, TestDb) {
    let test_db = TestDb::create().await;
    let app = app_for(&test_db, &[FIRST_ID, SECOND_ID]).await;
    (app, test_db)
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

fn graphql(body: Value) -> Request<Body> {
    Request::builder()
        .method(Method::POST)
        .uri(routes::GRAPHQL)
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(body.to_string()))
        .unwrap()
}

fn laptop_input() -> Value {
    json!({
        "title": "Laptop",
        "handle": "laptop",
        "description": "Light laptop for travel.",
        "priceCents": 50000,
        "inventoryQuantity": 10,
        "published": true
    })
}

fn phone_input() -> Value {
    json!({
        "title": "Phone",
        "handle": "phone",
        "priceCents": 30000,
        "inventoryQuantity": 5,
        "published": false
    })
}

fn create_request(input: Value) -> Value {
    json!({
        "query": CREATE_MUTATION,
        "variables": { "input": input }
    })
}

fn expected_laptop() -> Value {
    json!({
        "id": FIRST_ID,
        "title": "Laptop",
        "handle": "laptop",
        "description": "Light laptop for travel.",
        "priceCents": 50000,
        "inventoryQuantity": 10,
        "published": true,
        "publishedAt": NOW,
        "createdAt": NOW,
        "updatedAt": NOW
    })
}

fn expected_phone() -> Value {
    json!({
        "id": SECOND_ID,
        "title": "Phone",
        "handle": "phone",
        "description": null,
        "priceCents": 30000,
        "inventoryQuantity": 5,
        "published": false,
        "publishedAt": null,
        "createdAt": NOW,
        "updatedAt": NOW
    })
}

fn error_code(body: &Value) -> &str {
    body["errors"][0]["extensions"]["code"]
        .as_str()
        .expect("GraphQL error code")
}

#[tokio::test]
async fn products_query_starts_empty() {
    let (app, test_db) = test_app().await;

    let (status, body) = send(&app, graphql(json!({ "query": PRODUCTS_QUERY }))).await;

    assert_eq!(status, StatusCode::OK);
    assert!(body.get("errors").is_none());
    assert_eq!(body["data"]["products"], json!([]));
    test_db.drop().await;
}

#[tokio::test]
async fn product_create_mutation_returns_contract_shape() {
    let (app, test_db) = test_app().await;

    let (status, body) = send(&app, graphql(create_request(laptop_input()))).await;

    assert_eq!(status, StatusCode::OK);
    assert!(body.get("errors").is_none());
    assert_eq!(body["data"]["productCreate"], expected_laptop());
    test_db.drop().await;
}

#[tokio::test]
async fn products_query_returns_created_products_with_timestamps() {
    let (app, test_db) = test_app().await;

    send(&app, graphql(create_request(laptop_input()))).await;
    send(&app, graphql(create_request(phone_input()))).await;
    let (status, body) = send(&app, graphql(json!({ "query": PRODUCTS_QUERY }))).await;

    assert_eq!(status, StatusCode::OK);
    assert!(body.get("errors").is_none());
    assert_eq!(
        body["data"]["products"],
        json!([expected_laptop(), expected_phone()])
    );
    test_db.drop().await;
}

#[tokio::test]
async fn unpublished_create_keeps_null_description_and_published_at() {
    let (app, test_db) = test_app().await;

    let (status, body) = send(&app, graphql(create_request(phone_input()))).await;

    assert_eq!(status, StatusCode::OK);
    assert!(body.get("errors").is_none());
    assert_eq!(body["data"]["productCreate"]["description"], Value::Null);
    assert_eq!(body["data"]["productCreate"]["publishedAt"], Value::Null);
    assert_eq!(body["data"]["productCreate"]["createdAt"], NOW);
    assert_eq!(body["data"]["productCreate"]["updatedAt"], NOW);
    test_db.drop().await;
}

#[tokio::test]
async fn duplicate_handle_returns_chapter_error_code() {
    let (app, test_db) = test_app().await;

    send(&app, graphql(create_request(laptop_input()))).await;
    let (status, body) = send(&app, graphql(create_request(laptop_input()))).await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(error_code(&body), "duplicate_product_handle");
    assert!(
        body["errors"][0]["extensions"]["request_id"]
            .as_str()
            .is_some_and(|id| !id.is_empty())
    );
    test_db.drop().await;
}

#[tokio::test]
async fn blank_title_returns_validation_failed() {
    let (app, test_db) = test_app().await;
    let mut input = laptop_input();
    input["title"] = json!("  ");

    let (status, body) = send(&app, graphql(create_request(input))).await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(error_code(&body), "validation_failed");
    test_db.drop().await;
}

#[tokio::test]
async fn negative_price_returns_validation_failed() {
    let (app, test_db) = test_app().await;
    let mut input = laptop_input();
    input["priceCents"] = json!(-1);

    let (status, body) = send(&app, graphql(create_request(input))).await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(error_code(&body), "validation_failed");
    test_db.drop().await;
}
