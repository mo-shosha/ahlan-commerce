use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;

use api::{config::Config, constants::{env_keys, routes}, db, router, state::AppState};
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
use sqlx::{AssertSqlSafe, PgPool, postgres::PgConnectOptions};
use tower::ServiceExt;
use uuid::Uuid;

const FIRST_ID: &str = "018f7b2a-9f62-7d0c-8c4f-7a1f37c1d001";
const SECOND_ID: &str = "018f7b2a-9f62-7d0c-8c4f-7a1f37c1d002";
const NOW: &str = "2026-01-01T00:00:00Z";
// All tests connect at once, so connection setup can take longer than the API's request timeout.
const TEST_ACQUIRE_TIMEOUT: Duration = Duration::from_secs(30);
// Applied in this order, the same order as on a real database.
const SCHEMA_FILES: [(&str, &str); 2] = [
    ("initial-products.sql", include_str!("../../../db/initial-products.sql")),
    (
        "add-description-and-published-at.sql",
        include_str!("../../../db/add-description-and-published-at.sql"),
    ),
];

struct TestDb {
    database_url: String,
    schema: String,
}

impl TestDb {
    async fn create() -> Self {
        let database_url = std::env::var(env_keys::DATABASE_URL).unwrap_or_else(|_| {
            panic!("set {} to run the API tests against PostgreSQL", env_keys::DATABASE_URL)
        });
        let schema = format!("test_{}", Uuid::now_v7().simple());
        let test_db = Self { database_url, schema };

        let admin = test_db.admin_pool().await;
        sqlx::raw_sql(AssertSqlSafe(format!("create schema {}", test_db.schema)))
            .execute(&admin)
            .await
            .expect("create test schema");
        admin.close().await;

        let pool = test_db.connect().await;
        for (name, sql) in SCHEMA_FILES {
            sqlx::raw_sql(sql)
                .execute(&pool)
                .await
                .unwrap_or_else(|err| panic!("apply {name}: {err}"));
        }
        pool.close().await;

        test_db
    }

    async fn connect(&self) -> PgPool {
        let options = PgConnectOptions::from_str(&self.database_url)
            .expect("valid database url")
            .options([("search_path", self.schema.as_str())]);

        db::pool_options()
            .acquire_timeout(TEST_ACQUIRE_TIMEOUT)
            .connect_with(options)
            .await
            .expect("connect to test schema")
    }

    async fn admin_pool(&self) -> PgPool {
        db::pool_options()
            .max_connections(1)
            .acquire_timeout(TEST_ACQUIRE_TIMEOUT)
            .connect(&self.database_url)
            .await
            .expect("connect to PostgreSQL")
    }

    async fn drop(self) {
        let admin = self.admin_pool().await;
        sqlx::raw_sql(AssertSqlSafe(format!("drop schema {} cascade", self.schema)))
            .execute(&admin)
            .await
            .expect("drop test schema");
        admin.close().await;
    }
}

async fn app_for(test_db: &TestDb, ids: &[&str]) -> Router {
    let config = Config::from_lookup(|key| {
        (key == env_keys::DATABASE_URL).then(|| test_db.database_url.clone())
    })
    .expect("test config is valid");
    let catalog = ProductCatalog::new(
        Arc::new(FixedIds::new(ids.iter().copied())),
        Arc::new(FixedClock::from_rfc3339(NOW)),
    );
    router(AppState::new(config, test_db.connect().await, catalog))
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
        "description": "Light laptop for travel.",
        "price_cents": 50000,
        "inventory_quantity": 10,
        "published": true
    })
}

fn phone_request() -> Value {
    json!({
        "title": "Phone",
        "handle": "phone",
        "price_cents": 30000,
        "inventory_quantity": 5,
        "published": false
    })
}

async fn assert_validation_failed(request: Value) {
    let (app, test_db) = test_app().await;

    let (status, body) = send(&app, post_json(routes::PRODUCTS, request)).await;

    assert_eq!(status, StatusCode::BAD_REQUEST);
    assert_eq!(body["error"]["code"], "validation_failed");
    assert!(body["error"]["request_id"].as_str().is_some_and(|id| !id.is_empty()));
    test_db.drop().await;
}

#[tokio::test]
async fn health_returns_ok() {
    let (app, test_db) = test_app().await;

    let (status, body) = send(&app, get(routes::HEALTH)).await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(body, json!({ "status": "ok" }));
    test_db.drop().await;
}

#[tokio::test]
async fn list_products_starts_empty() {
    let (app, test_db) = test_app().await;

    let (status, body) = send(&app, get(routes::PRODUCTS)).await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(body, json!({ "products": [] }));
    test_db.drop().await;
}

#[tokio::test]
async fn create_published_product_returns_contract_shape() {
    let (app, test_db) = test_app().await;

    let (status, body) = send(&app, post_json(routes::PRODUCTS, laptop_request())).await;

    assert_eq!(status, StatusCode::CREATED);
    assert_eq!(
        body,
        json!({
            "product": {
                "id": FIRST_ID,
                "title": "Laptop",
                "handle": "laptop",
                "description": "Light laptop for travel.",
                "price_cents": 50000,
                "inventory_quantity": 10,
                "published": true,
                "published_at": NOW,
                "created_at": NOW,
                "updated_at": NOW
            }
        })
    );
    test_db.drop().await;
}

#[tokio::test]
async fn unpublished_product_without_description_stores_nulls() {
    let (app, test_db) = test_app().await;

    let (status, body) = send(&app, post_json(routes::PRODUCTS, phone_request())).await;

    assert_eq!(status, StatusCode::CREATED);
    assert_eq!(body["product"]["description"], Value::Null);
    assert_eq!(body["product"]["published"], false);
    assert_eq!(body["product"]["published_at"], Value::Null);
    test_db.drop().await;
}

#[tokio::test]
async fn null_description_is_stored_as_null() {
    let (app, test_db) = test_app().await;
    let mut request = laptop_request();
    request["description"] = Value::Null;

    let (status, body) = send(&app, post_json(routes::PRODUCTS, request)).await;

    assert_eq!(status, StatusCode::CREATED);
    assert_eq!(body["product"]["description"], Value::Null);
    test_db.drop().await;
}

#[tokio::test]
async fn list_returns_new_columns_in_creation_order() {
    let (app, test_db) = test_app().await;

    send(&app, post_json(routes::PRODUCTS, laptop_request())).await;
    send(&app, post_json(routes::PRODUCTS, phone_request())).await;
    let (status, body) = send(&app, get(routes::PRODUCTS)).await;

    assert_eq!(status, StatusCode::OK);
    let products = body["products"].as_array().expect("products is an array");
    assert_eq!(products.len(), 2);

    assert_eq!(products[0]["id"], FIRST_ID);
    assert_eq!(products[0]["handle"], "laptop");
    assert_eq!(products[0]["description"], "Light laptop for travel.");
    assert_eq!(products[0]["published_at"], NOW);
    assert_eq!(products[0]["created_at"], NOW);
    assert_eq!(products[0]["updated_at"], NOW);

    assert_eq!(products[1]["id"], SECOND_ID);
    assert_eq!(products[1]["handle"], "phone");
    assert_eq!(products[1]["description"], Value::Null);
    assert_eq!(products[1]["published_at"], Value::Null);
    test_db.drop().await;
}

#[tokio::test]
async fn products_survive_an_api_restart() {
    let test_db = TestDb::create().await;
    let (_, created) = send(
        &app_for(&test_db, &[FIRST_ID]).await,
        post_json(routes::PRODUCTS, laptop_request()),
    )
    .await;

    let restarted = app_for(&test_db, &[]).await;
    let (status, body) = send(&restarted, get(routes::PRODUCTS)).await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(body, json!({ "products": [created["product"]] }));
    test_db.drop().await;
}

#[tokio::test]
async fn duplicate_handle_returns_conflict() {
    let (app, test_db) = test_app().await;

    send(&app, post_json(routes::PRODUCTS, laptop_request())).await;
    let (status, body) = send(&app, post_json(routes::PRODUCTS, laptop_request())).await;

    assert_eq!(status, StatusCode::CONFLICT);
    assert_eq!(body["error"]["code"], "duplicate_product_handle");
    test_db.drop().await;
}

#[tokio::test]
async fn blank_title_returns_validation_error() {
    let mut request = laptop_request();
    request["title"] = json!("  ");

    assert_validation_failed(request).await;
}

#[tokio::test]
async fn missing_title_returns_validation_error() {
    let mut request = laptop_request();
    request.as_object_mut().unwrap().remove("title");

    assert_validation_failed(request).await;
}

#[tokio::test]
async fn handle_that_is_not_lowercase_url_safe_returns_validation_error() {
    for handle in ["Coffee-Mug", "coffee mug", "coffee/mug"] {
        let mut request = laptop_request();
        request["handle"] = json!(handle);

        assert_validation_failed(request).await;
    }
}

#[tokio::test]
async fn negative_price_returns_validation_error() {
    let mut request = laptop_request();
    request["price_cents"] = json!(-1);

    assert_validation_failed(request).await;
}

#[tokio::test]
async fn negative_inventory_returns_validation_error() {
    let mut request = laptop_request();
    request["inventory_quantity"] = json!(-1);

    assert_validation_failed(request).await;
}

#[tokio::test]
async fn price_above_integer_column_returns_validation_error() {
    let (app, test_db) = test_app().await;
    let mut request = laptop_request();
    request["price_cents"] = json!(u32::MAX);

    let (status, body) = send(&app, post_json(routes::PRODUCTS, request)).await;

    assert_eq!(status, StatusCode::BAD_REQUEST);
    assert_eq!(body["error"]["code"], "validation_failed");
    assert_eq!(body["error"]["message"], "price_cents must be at most 2147483647.");
    test_db.drop().await;
}

#[tokio::test]
async fn unknown_route_returns_not_found_error() {
    let (app, test_db) = test_app().await;

    let (status, body) = send(&app, get("/api/does-not-exist")).await;

    assert_eq!(status, StatusCode::NOT_FOUND);
    assert_eq!(body["error"]["code"], "not_found");
    assert!(body["error"]["request_id"].as_str().is_some_and(|id| !id.is_empty()));
    test_db.drop().await;
}

#[tokio::test]
async fn internal_failure_returns_safe_public_error() {
    let (app, test_db) = test_app().await;

    let response = app
        .oneshot(get(routes::SIMULATED_INTERNAL_FAILURE))
        .await
        .expect("request failed");

    let status = response.status();
    let header_request_id = response
        .headers()
        .get("x-request-id")
        .expect("x-request-id header is set")
        .to_str()
        .unwrap()
        .to_string();
    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    let raw_body = String::from_utf8(bytes.to_vec()).unwrap();
    let body: Value = serde_json::from_str(&raw_body).expect("response body is JSON");

    assert_eq!(status, StatusCode::INTERNAL_SERVER_ERROR);
    assert!(!header_request_id.is_empty());

    // Exact match: the body has only the public fields and nothing extra.
    assert_eq!(
        body,
        json!({
            "error": {
                "code": "internal_error",
                "message": "An unexpected server error occurred.",
                "request_id": header_request_id
            }
        })
    );

    for internal_detail in ["Inventory sync", "invalid digit", "ParseIntError", ".rs:"] {
        assert!(
            !raw_body.contains(internal_detail),
            "response leaked internal detail: {internal_detail}"
        );
    }
    test_db.drop().await;
}
