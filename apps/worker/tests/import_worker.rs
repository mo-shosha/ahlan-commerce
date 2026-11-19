use std::sync::Arc;

use catalog::{
    ImportJobService, NewProduct, ProductCatalog, STATUS_FAILED, STATUS_SUCCEEDED,
    testing::{FixedClock, FixedIds},
};
use db::testing::TestDb;
use worker::Worker;

const JOB_ID: &str = "018f7b2a-9f62-7d0c-8c4f-7a1f37c1d101";
const PRODUCT_A: &str = "018f7b2a-9f62-7d0c-8c4f-7a1f37c1d102";
const PRODUCT_B: &str = "018f7b2a-9f62-7d0c-8c4f-7a1f37c1d103";
const PRODUCT_C: &str = "018f7b2a-9f62-7d0c-8c4f-7a1f37c1d104";
const NOW: &str = "2026-01-01T00:00:00Z";

const TWO_PRODUCTS: &str = r#"{
  "products": [
    {
      "title": "Coffee Mug",
      "handle": "coffee-mug",
      "description": "Ceramic mug for daily coffee.",
      "price_cents": 2500,
      "inventory_quantity": 12,
      "published": true
    },
    {
      "title": "Tea Cup",
      "handle": "tea-cup",
      "price_cents": 1800,
      "inventory_quantity": 20,
      "published": false
    }
  ]
}"#;

fn write_import_file(dir: &tempfile::TempDir, contents: &str) -> String {
    let path = dir.path().join("products-import.json");
    std::fs::write(&path, contents).expect("write fixture");
    path.to_string_lossy().into_owned()
}

struct Harness {
    test_db: TestDb,
    worker: Worker,
    catalog: ProductCatalog,
    import_jobs: ImportJobService,
    pool: db::Pool,
}

async fn harness(ids: &[&str]) -> Harness {
    let test_db = TestDb::create().await;
    let pool = test_db.pool().await;
    let id_gen = Arc::new(FixedIds::new(ids.iter().copied()));
    let clock = Arc::new(FixedClock::from_rfc3339(NOW));
    let catalog = ProductCatalog::new(id_gen.clone(), clock.clone());
    let import_jobs = ImportJobService::new(id_gen, clock);
    let worker = Worker::new(pool.clone(), catalog.clone(), import_jobs.clone());

    Harness {
        test_db,
        worker,
        catalog,
        import_jobs,
        pool,
    }
}

#[tokio::test]
async fn worker_imports_products_and_marks_job_succeeded() {
    let dir = tempfile::tempdir().expect("tempdir");
    let path = write_import_file(&dir, TWO_PRODUCTS);
    let h = harness(&[JOB_ID, PRODUCT_A, PRODUCT_B]).await;

    let job = h
        .import_jobs
        .enqueue(&h.pool, path)
        .await
        .expect("enqueue");

    assert!(h.worker.tick().await.expect("tick"));

    let finished = h.import_jobs.get(&h.pool, job.id).await.expect("get job");
    assert_eq!(finished.status, STATUS_SUCCEEDED);
    assert_eq!(finished.attempts, 1);
    assert_eq!(finished.last_error, None);

    let products = h.catalog.list_products(&h.pool).await.expect("list");
    assert_eq!(products.len(), 2);
    assert_eq!(products[0].handle, "coffee-mug");
    assert!(products[0].published_at.is_some());
    assert_eq!(products[1].handle, "tea-cup");
    assert_eq!(products[1].published_at, None);

    h.test_db.drop().await;
}

#[tokio::test]
async fn missing_input_file_marks_job_failed() {
    let h = harness(&[JOB_ID]).await;

    let job = h
        .import_jobs
        .enqueue(&h.pool, "fixtures/does-not-exist.json".to_string())
        .await
        .expect("enqueue");

    assert!(h.worker.tick().await.expect("tick"));

    let finished = h.import_jobs.get(&h.pool, job.id).await.expect("get job");
    assert_eq!(finished.status, STATUS_FAILED);
    assert_eq!(finished.attempts, 1);
    let error = finished.last_error.expect("last_error");
    assert!(error.contains("not found"), "{error}");

    h.test_db.drop().await;
}

#[tokio::test]
async fn duplicate_handle_marks_job_failed_with_handle() {
    let dir = tempfile::tempdir().expect("tempdir");
    let path = write_import_file(&dir, TWO_PRODUCTS);
    let h = harness(&[PRODUCT_C, JOB_ID, PRODUCT_A, PRODUCT_B]).await;

    h.catalog
        .create_product(
            &h.pool,
            NewProduct {
                title: "Existing Mug".to_string(),
                handle: "coffee-mug".to_string(),
                description: None,
                price_cents: 1000,
                inventory_quantity: 1,
                published: false,
            },
        )
        .await
        .expect("seed product");

    let job = h
        .import_jobs
        .enqueue(&h.pool, path)
        .await
        .expect("enqueue");

    assert!(h.worker.tick().await.expect("tick"));

    let finished = h.import_jobs.get(&h.pool, job.id).await.expect("get job");
    assert_eq!(finished.status, STATUS_FAILED);
    let error = finished.last_error.expect("last_error");
    assert!(error.contains("coffee-mug"), "{error}");

    h.test_db.drop().await;
}

#[tokio::test]
async fn failed_job_can_be_retried_until_max_attempts() {
    let h = harness(&[JOB_ID]).await;
    let job = h
        .import_jobs
        .enqueue(&h.pool, "fixtures/does-not-exist.json".to_string())
        .await
        .expect("enqueue");

    for expected_attempts in 1..=3 {
        if expected_attempts > 1 {
            h.import_jobs
                .requeue(&h.pool, job.id)
                .await
                .expect("requeue keeps attempts");
        }

        assert!(h.worker.tick().await.expect("tick"));
        let current = h.import_jobs.get(&h.pool, job.id).await.expect("get");
        assert_eq!(current.status, STATUS_FAILED);
        assert_eq!(current.attempts, expected_attempts);
    }

    // After 3 attempts, even if requeued, the worker must not claim it again.
    h.import_jobs
        .requeue(&h.pool, job.id)
        .await
        .expect("requeue after max");
    assert!(!h.worker.tick().await.expect("idle tick"));
    let current = h.import_jobs.get(&h.pool, job.id).await.expect("get");
    assert_eq!(current.attempts, 3);

    h.test_db.drop().await;
}
