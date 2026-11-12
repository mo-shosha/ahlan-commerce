use db::products::ProductRecord;

use crate::{CatalogError, Product, ProductId};

pub(crate) fn to_record(product: &Product) -> Result<ProductRecord, CatalogError> {
    Ok(ProductRecord {
        id: product.id.as_uuid(),
        title: product.title.clone(),
        handle: product.handle.clone(),
        description: product.description.clone(),
        price_cents: to_db_int("price_cents", product.price_cents)?,
        inventory_quantity: to_db_int("inventory_quantity", product.inventory_quantity)?,
        published: product.published,
        published_at: product.published_at,
        created_at: product.created_at,
        updated_at: product.updated_at,
    })
}

pub(crate) fn from_record(record: ProductRecord) -> Result<Product, CatalogError> {
    Ok(Product {
        id: ProductId::from_uuid(record.id),
        price_cents: from_db_int("price_cents", record.price_cents)?,
        inventory_quantity: from_db_int("inventory_quantity", record.inventory_quantity)?,
        title: record.title,
        handle: record.handle,
        description: record.description,
        published: record.published,
        published_at: record.published_at,
        created_at: record.created_at,
        updated_at: record.updated_at,
    })
}

pub(crate) fn from_records(records: Vec<ProductRecord>) -> Result<Vec<Product>, CatalogError> {
    records.into_iter().map(from_record).collect()
}

fn to_db_int(field: &'static str, value: u32) -> Result<i32, CatalogError> {
    i32::try_from(value).map_err(|_| CatalogError::ValueTooLarge { field, max: i32::MAX })
}

fn from_db_int(field: &'static str, value: i32) -> Result<u32, CatalogError> {
    u32::try_from(value).map_err(|_| CatalogError::InvalidStoredValue { field, value })
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    fn record() -> ProductRecord {
        let now = "2026-01-01T00:00:00Z".parse().unwrap();
        ProductRecord {
            id: Uuid::nil(),
            title: "Mug".to_string(),
            handle: "mug".to_string(),
            description: None,
            price_cents: 1_000,
            inventory_quantity: 3,
            published: true,
            published_at: Some(now),
            created_at: now,
            updated_at: now,
        }
    }

    #[test]
    fn record_round_trips_through_product() {
        let product = from_record(record()).expect("valid record");

        assert_eq!(to_record(&product).expect("fits in the columns"), record());
    }

    #[test]
    fn negative_stored_quantity_is_rejected() {
        let error = from_record(ProductRecord { inventory_quantity: -1, ..record() }).unwrap_err();

        assert!(matches!(
            error,
            CatalogError::InvalidStoredValue { field: "inventory_quantity", value: -1 }
        ));
    }

    #[test]
    fn price_above_integer_column_is_rejected() {
        let mut product = from_record(record()).expect("valid record");
        product.price_cents = u32::MAX;

        let error = to_record(&product).unwrap_err();

        assert!(matches!(error, CatalogError::ValueTooLarge { field: "price_cents", .. }));
    }
}
