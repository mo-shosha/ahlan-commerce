use std::sync::Arc;

use api::{config::Config, constants::{env_keys, routes}, router, state::AppState};
use axum::{
    Router,
    body::Body,
    http::{Method, Request, StatusCode, header},
};
use catalog::{
    ImportJobService, ProductCatalog, STATUS_QUEUED,
    testing::{FixedClock, FixedIds},
};
use db::testing::TestDb;
use http_body_util::BodyExt;
use serde_json::{Value, json};
use tower::ServiceExt;

const JOB_ID: &str = "018f7b2a-9f62-7d0c-8c4f-7a1f37c1d0aa";
const NOW: &str = "2026-01-01T00:00:00Z";

async fn test_app() -> (Router, TestDb) {
    let test_db = TestDb::create().await;
    let config = Config::from_lookup(|key| {
        (key == env_keys::DATABASE_URL).then(|| test_db.database_url().to_string())
    })
    .expect("test config is valid");
    let ids = Arc::new(FixedIds::new([JOB_ID]));
    let clock = Arc::new(FixedClock::from_rfc3339(NOW));
    let catalog = ProductCatalog::new(ids.clone(), clock.clone());
    let import_jobs = ImportJobService::new(ids, clock);
    let app = router(AppState::new(config, test_db.pool().await, catalog, import_jobs));
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

fn post_json(uri: &str, body: Value) -> Request<Body> {
    Request::builder()
        .method(Method::POST)
        .uri(uri)
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(body.to_string()))
        .unwrap()
}

#[tokio::test]
async fn enqueue_returns_accepted_queued_job() {
    let (app, test_db) = test_app().await;

    let (status, body) = send(
        &app,
        post_json(
            routes::IMPORT_JOBS,
            json!({ "input_path": "fixtures/products-import.json" }),
        ),
    )
    .await;

    assert_eq!(status, StatusCode::ACCEPTED);
    assert_eq!(body["job"]["id"], JOB_ID);
    assert_eq!(body["job"]["status"], STATUS_QUEUED);
    test_db.drop().await;
}

#[tokio::test]
async fn enqueue_accepts_missing_file_path() {
    // The API only checks the request shape. Missing files fail later in the worker.
    let (app, test_db) = test_app().await;

    let (status, body) = send(
        &app,
        post_json(
            routes::IMPORT_JOBS,
            json!({ "input_path": "fixtures/does-not-exist.json" }),
        ),
    )
    .await;

    assert_eq!(status, StatusCode::ACCEPTED);
    assert_eq!(body["job"]["status"], STATUS_QUEUED);
    test_db.drop().await;
}

#[tokio::test]
async fn enqueue_rejects_empty_input_path() {
    let (app, test_db) = test_app().await;

    let (status, body) = send(
        &app,
        post_json(routes::IMPORT_JOBS, json!({ "input_path": "   " })),
    )
    .await;

    assert_eq!(status, StatusCode::BAD_REQUEST);
    assert_eq!(body["error"]["code"], "validation_failed");
    assert!(body["error"]["request_id"].as_str().is_some_and(|id| !id.is_empty()));
    test_db.drop().await;
}

#[tokio::test]
async fn enqueue_rejects_invalid_json_body() {
    let (app, test_db) = test_app().await;

    let (status, body) = send(
        &app,
        post_json(routes::IMPORT_JOBS, json!({ "path": "fixtures/products-import.json" })),
    )
    .await;

    assert_eq!(status, StatusCode::BAD_REQUEST);
    assert_eq!(body["error"]["code"], "validation_failed");
    test_db.drop().await;
}
