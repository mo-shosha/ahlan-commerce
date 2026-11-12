use chrono::{DateTime, Utc};
use db::DbError;
use db::products::{self, ProductPublication, ProductRecord};
use db::testing::TestDb;
use uuid::Uuid;

const FIRST_ID: &str = "018f7b2a-9f62-7d0c-8c4f-7a1f37c1d001";
const SECOND_ID: &str = "018f7b2a-9f62-7d0c-8c4f-7a1f37c1d002";
const THIRD_ID: &str = "018f7b2a-9f62-7d0c-8c4f-7a1f37c1d003";

fn at(rfc3339: &str) -> DateTime<Utc> {
    rfc3339.parse().expect("valid timestamp")
}

fn id(raw: &str) -> Uuid {
    raw.parse().expect("valid uuid")
}

fn record(raw_id: &str, handle: &str, created_at: &str, published_at: Option<&str>) -> ProductRecord {
    ProductRecord {
        id: id(raw_id),
        title: handle.to_uppercase(),
        handle: handle.to_string(),
        description: Some(format!("About {handle}.")),
        price_cents: 1_000,
        inventory_quantity: 3,
        published: published_at.is_some(),
        published_at: published_at.map(at),
        created_at: at(created_at),
        updated_at: at(created_at),
    }
}

#[tokio::test]
async fn create_returns_every_product_field() {
    let test_db = TestDb::create().await;
    let db = test_db.pool().await;
    let draft = ProductRecord {
        description: None,
        ..record(FIRST_ID, "mug", "2026-01-01T00:00:00Z", None)
    };

    let created = products::create_product(&db, &draft).await.expect("create product");

    assert_eq!(created, draft);
    test_db.drop().await;
}

#[tokio::test]
async fn create_rejects_a_duplicate_handle() {
    let test_db = TestDb::create().await;
    let db = test_db.pool().await;
    products::create_product(&db, &record(FIRST_ID, "mug", "2026-01-01T00:00:00Z", None))
        .await
        .expect("create first product");

    let error = products::create_product(&db, &record(SECOND_ID, "mug", "2026-01-02T00:00:00Z", None))
        .await
        .unwrap_err();

    assert!(matches!(error, DbError::DuplicateProductHandle), "got {error:?}");
    test_db.drop().await;
}

#[tokio::test]
async fn list_orders_by_created_at_then_id() {
    let test_db = TestDb::create().await;
    let db = test_db.pool().await;
    let later = record(FIRST_ID, "later", "2026-01-02T00:00:00Z", None);
    let tie_second = record(THIRD_ID, "tie-second", "2026-01-01T00:00:00Z", None);
    let tie_first = record(SECOND_ID, "tie-first", "2026-01-01T00:00:00Z", Some("2026-01-01T00:00:00Z"));
    for product in [&later, &tie_second, &tie_first] {
        products::create_product(&db, product).await.expect("create product");
    }

    let listed = products::list_products(&db).await.expect("list products");

    assert_eq!(listed, vec![tie_first, tie_second, later]);
    test_db.drop().await;
}

#[tokio::test]
async fn published_list_skips_drafts_and_puts_newest_publication_first() {
    let test_db = TestDb::create().await;
    let db = test_db.pool().await;
    let published_first = record(FIRST_ID, "old-news", "2026-01-01T00:00:00Z", Some("2026-01-05T00:00:00Z"));
    let draft = record(SECOND_ID, "draft", "2026-01-02T00:00:00Z", None);
    let published_last = record(THIRD_ID, "fresh", "2026-01-03T00:00:00Z", Some("2026-01-09T00:00:00Z"));
    for product in [&published_first, &draft, &published_last] {
        products::create_product(&db, product).await.expect("create product");
    }

    let listed = products::list_published_products(&db).await.expect("list published products");

    assert_eq!(listed, vec![published_last, published_first]);
    test_db.drop().await;
}

#[tokio::test]
async fn update_publication_sets_only_publication_fields() {
    let test_db = TestDb::create().await;
    let db = test_db.pool().await;
    let draft = record(FIRST_ID, "mug", "2026-01-01T00:00:00Z", None);
    products::create_product(&db, &draft).await.expect("create product");
    let now = at("2026-02-01T00:00:00Z");

    let updated = products::update_product_publication(
        &db,
        &ProductPublication { id: draft.id, published: true, published_at: Some(now), updated_at: now },
    )
    .await
    .expect("update publication")
    .expect("product exists");

    assert_eq!(
        updated,
        ProductRecord { published: true, published_at: Some(now), updated_at: now, ..draft }
    );
    test_db.drop().await;
}

#[tokio::test]
async fn update_publication_of_unknown_product_returns_none() {
    let test_db = TestDb::create().await;
    let db = test_db.pool().await;
    let now = at("2026-02-01T00:00:00Z");

    let updated = products::update_product_publication(
        &db,
        &ProductPublication { id: id(FIRST_ID), published: true, published_at: Some(now), updated_at: now },
    )
    .await
    .expect("update publication");

    assert_eq!(updated, None);
    test_db.drop().await;
}
