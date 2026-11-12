//! Product persistence. Each function wraps one query from `db/queries/products/`.

use chrono::{DateTime, Utc};
use db_generated::client::Params;
use db_generated::queries::products::{
    create_product, list_products, list_published_products, update_product_publication,
};
use db_generated::tokio_postgres;
use uuid::Uuid;

use crate::{DbError, Pool};

const HANDLE_UNIQUE_CONSTRAINT: &str = "products_handle_key";

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProductRecord {
    pub id: Uuid,
    pub title: String,
    pub handle: String,
    pub description: Option<String>,
    pub price_cents: i32,
    pub inventory_quantity: i32,
    pub published: bool,
    pub published_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ProductPublication {
    pub id: Uuid,
    pub published: bool,
    pub published_at: Option<DateTime<Utc>>,
    pub updated_at: DateTime<Utc>,
}

pub async fn create_product(db: &Pool, product: &ProductRecord) -> Result<ProductRecord, DbError> {
    let client = db.get().await?;

    let row = create_product::create_product()
        .params(
            &client,
            &create_product::CreateProductParams {
                id: product.id,
                title: &product.title,
                handle: &product.handle,
                description: product.description.as_deref(),
                price_cents: product.price_cents,
                inventory_quantity: product.inventory_quantity,
                published: product.published,
                published_at: product.published_at,
                created_at: product.created_at,
                updated_at: product.updated_at,
            },
        )
        .one()
        .await
        .map_err(map_write_error)?;

    Ok(row.into())
}

pub async fn list_products(db: &Pool) -> Result<Vec<ProductRecord>, DbError> {
    let client = db.get().await?;

    let rows = list_products::list_products().bind(&client).all().await?;

    Ok(rows.into_iter().map(ProductRecord::from).collect())
}

pub async fn list_published_products(db: &Pool) -> Result<Vec<ProductRecord>, DbError> {
    let client = db.get().await?;

    let rows = list_published_products::list_published_products()
        .bind(&client)
        .all()
        .await?;

    Ok(rows.into_iter().map(ProductRecord::from).collect())
}

pub async fn update_product_publication(
    db: &Pool,
    change: &ProductPublication,
) -> Result<Option<ProductRecord>, DbError> {
    let client = db.get().await?;

    let row = update_product_publication::update_product_publication()
        .params(
            &client,
            &update_product_publication::UpdateProductPublicationParams {
                id: change.id,
                published: change.published,
                published_at: change.published_at,
                updated_at: change.updated_at,
            },
        )
        .opt()
        .await?;

    Ok(row.map(ProductRecord::from))
}

fn map_write_error(error: tokio_postgres::Error) -> DbError {
    let duplicate_handle = error
        .as_db_error()
        .is_some_and(|db_error| db_error.constraint() == Some(HANDLE_UNIQUE_CONSTRAINT));

    if duplicate_handle {
        DbError::DuplicateProductHandle
    } else {
        error.into()
    }
}

macro_rules! product_record_from_rows {
    ($($row:ty),+ $(,)?) => {$(
        impl From<$row> for ProductRecord {
            fn from(row: $row) -> Self {
                Self {
                    id: row.id,
                    title: row.title,
                    handle: row.handle,
                    description: row.description,
                    price_cents: row.price_cents,
                    inventory_quantity: row.inventory_quantity,
                    published: row.published,
                    published_at: row.published_at,
                    created_at: row.created_at,
                    updated_at: row.updated_at,
                }
            }
        }
    )+};
}

product_record_from_rows!(
    create_product::CreateProduct,
    list_products::ListProducts,
    list_published_products::ListPublishedProducts,
    update_product_publication::UpdateProductPublication,
);
