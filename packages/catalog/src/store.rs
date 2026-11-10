use sqlx::{PgPool, Row, postgres::PgRow};

use crate::{CatalogError, Product, ProductId};

const HANDLE_UNIQUE_CONSTRAINT: &str = "products_handle_key";

pub(crate) async fn insert_product(db: &PgPool, product: &Product) -> Result<Product, CatalogError> {
    let price_cents = to_db_int("price_cents", product.price_cents)?;
    let inventory_quantity = to_db_int("inventory_quantity", product.inventory_quantity)?;

    let row = sqlx::query(
        "insert into products
            (id, title, handle, price_cents, inventory_quantity, published, created_at, updated_at)
         values ($1, $2, $3, $4, $5, $6, $7, $8)
         returning id, title, handle, price_cents, inventory_quantity, published, created_at, updated_at",
    )
    .bind(product.id.as_uuid())
    .bind(&product.title)
    .bind(&product.handle)
    .bind(price_cents)
    .bind(inventory_quantity)
    .bind(product.published)
    .bind(product.created_at)
    .bind(product.updated_at)
    .fetch_one(db)
    .await
    .map_err(|error| match &error {
        sqlx::Error::Database(db_error)
            if db_error.constraint() == Some(HANDLE_UNIQUE_CONSTRAINT) =>
        {
            CatalogError::DuplicateProductHandle
        }
        _ => CatalogError::Storage(error),
    })?;

    Ok(product_from_row(&row)?)
}

pub(crate) async fn list_products(db: &PgPool) -> Result<Vec<Product>, CatalogError> {
    let rows = sqlx::query(
        "select id, title, handle, price_cents, inventory_quantity, published, created_at, updated_at
         from products
         order by created_at, id",
    )
    .fetch_all(db)
    .await?;

    let products = rows
        .iter()
        .map(product_from_row)
        .collect::<Result<_, _>>()?;

    Ok(products)
}

fn product_from_row(row: &PgRow) -> Result<Product, sqlx::Error> {
    Ok(Product {
        id: ProductId::from_uuid(row.try_get("id")?),
        title: row.try_get("title")?,
        handle: row.try_get("handle")?,
        price_cents: from_db_int(row, "price_cents")?,
        inventory_quantity: from_db_int(row, "inventory_quantity")?,
        published: row.try_get("published")?,
        created_at: row.try_get("created_at")?,
        updated_at: row.try_get("updated_at")?,
    })
}


fn to_db_int(field: &'static str, value: u32) -> Result<i32, CatalogError> {
    i32::try_from(value).map_err(|_| CatalogError::ValueTooLarge { field, max: i32::MAX })
}

fn from_db_int(row: &PgRow, column: &str) -> Result<u32, sqlx::Error> {
    let value: i32 = row.try_get(column)?;
    u32::try_from(value).map_err(|source| sqlx::Error::ColumnDecode {
        index: column.to_string(),
        source: Box::new(source),
    })
}
