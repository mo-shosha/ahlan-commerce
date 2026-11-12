use std::sync::Arc;

use db::Pool;
use db::products::{self, ProductPublication};

use crate::product::published_at;
use crate::records::{from_record, from_records, to_record};
use crate::{Clock, IdGenerator, NewProduct, Product, ProductId, CatalogError};


pub struct ProductCatalog {
    ids: Arc<dyn IdGenerator>,
    clock: Arc<dyn Clock>,
}

impl ProductCatalog {
    pub fn new(ids: Arc<dyn IdGenerator>, clock: Arc<dyn Clock>) -> Self {
        Self { ids, clock }
    }


    pub async fn create_product(&self, db: &Pool, input: NewProduct) -> Result<Product, CatalogError> {
        let product = self.new_product(input)?;

        let created = products::create_product(db, &to_record(&product)?).await?;
        from_record(created)
    }


    pub async fn list_products(&self, db: &Pool) -> Result<Vec<Product>, CatalogError> {
        from_records(products::list_products(db).await?)
    }

    pub async fn list_published_products(&self, db: &Pool) -> Result<Vec<Product>, CatalogError> {
        from_records(products::list_published_products(db).await?)
    }

    pub async fn set_product_publication(
        &self,
        db: &Pool,
        product_id: &str,
        published: bool,
    ) -> Result<Product, CatalogError> {
        let product_id = parse_product_id(product_id)?;
        let now = self.clock.now();
        let change = ProductPublication {
            id: product_id.as_uuid(),
            published,
            published_at: published_at(published, now),
            updated_at: now,
        };

        let updated = products::update_product_publication(db, &change)
            .await?
            .ok_or(CatalogError::ProductNotFound)?;
        from_record(updated)
    }

    fn new_product(&self, input: NewProduct) -> Result<Product, CatalogError> {
        validate(&input)?;

        Ok(Product::create(
            self.ids.new_product_id(),
            input,
            self.clock.now(),
        ))
    }
}

fn validate(input: &NewProduct) -> Result<(), CatalogError> {
    if input.title.trim().is_empty() {
        return Err(CatalogError::ValidationFailed);
    }

    if !is_url_safe_handle(&input.handle) {
        return Err(CatalogError::InvalidHandle);
    }

    Ok(())
}

fn parse_product_id(raw: &str) -> Result<ProductId, CatalogError> {
    uuid::Uuid::parse_str(raw)
        .map(ProductId::from_uuid)
        .map_err(|_| CatalogError::InvalidProductId)
}

fn is_url_safe_handle(handle: &str) -> bool {
    !handle.is_empty()
        && !handle.starts_with('-')
        && !handle.ends_with('-')
        && !handle.contains("--")
        && handle
            .bytes()
            .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'-')
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testing::{FixedClock, FixedIds};

    const FIRST_ID: &str = "018f7b2a-9f62-7d0c-8c4f-7a1f37c1d001";
    const SECOND_ID: &str = "018f7b2a-9f62-7d0c-8c4f-7a1f37c1d002";
    const NOW: &str = "2026-01-01T00:00:00Z";

    fn catalog() -> ProductCatalog {
        ProductCatalog::new(
            Arc::new(FixedIds::new([FIRST_ID, SECOND_ID])),
            Arc::new(FixedClock::from_rfc3339(NOW)),
        )
    }

    fn laptop() -> NewProduct {
        NewProduct {
            title: "Laptop".to_string(),
            handle: "laptop".to_string(),
            description: Some("Light laptop for travel.".to_string()),
            price_cents: 50_000,
            inventory_quantity: 10,
            published: true,
        }
    }

    fn phone() -> NewProduct {
        NewProduct {
            title: "Phone".to_string(),
            handle: "phone".to_string(),
            description: None,
            price_cents: 30_000,
            inventory_quantity: 5,
            published: false,
        }
    }

    #[test]
    fn new_product_copies_input_fields() {
        let product = catalog().new_product(laptop()).expect("laptop is valid");

        assert_eq!(product.title, "Laptop");
        assert_eq!(product.handle, "laptop");
        assert_eq!(product.description.as_deref(), Some("Light laptop for travel."));
        assert_eq!(product.price_cents, 50_000);
        assert_eq!(product.inventory_quantity, 10);
        assert!(product.published);
    }

    #[test]
    fn new_product_assigns_id_from_generator() {
        let product = catalog().new_product(laptop()).expect("laptop is valid");

        assert_eq!(product.id.to_string(), FIRST_ID);
    }

    #[test]
    fn new_product_sets_both_timestamps_from_clock() {
        let product = catalog().new_product(laptop()).expect("laptop is valid");
        let now = FixedClock::from_rfc3339(NOW).now();

        assert_eq!(product.created_at, now);
        assert_eq!(product.updated_at, now);
    }

    #[test]
    fn each_new_product_gets_the_next_id() {
        let catalog = catalog();

        let first = catalog.new_product(laptop()).expect("laptop is valid");
        let second = catalog.new_product(phone()).expect("phone is valid");

        assert_eq!(first.id.to_string(), FIRST_ID);
        assert_eq!(second.id.to_string(), SECOND_ID);
    }

    #[test]
    fn new_product_rejects_blank_title() {
        let blank = NewProduct { title: "   ".to_string(), ..laptop() };

        let error = catalog().new_product(blank).unwrap_err();

        assert!(matches!(error, CatalogError::ValidationFailed));
    }

    #[test]
    fn published_product_gets_published_at_from_clock() {
        let product = catalog().new_product(laptop()).expect("laptop is valid");

        assert_eq!(product.published_at, Some(FixedClock::from_rfc3339(NOW).now()));
    }

    #[test]
    fn unpublished_product_has_no_published_at() {
        let product = catalog().new_product(phone()).expect("phone is valid");

        assert_eq!(product.published_at, None);
    }

    #[test]
    fn new_product_accepts_url_safe_handles() {
        for handle in ["coffee-mug", "mug2", "a-1-b"] {
            let input = NewProduct { handle: handle.to_string(), ..laptop() };

            assert!(catalog().new_product(input).is_ok(), "{handle:?} should be accepted");
        }
    }

    #[test]
    fn new_product_rejects_handles_that_are_not_lowercase_url_safe() {
        for handle in ["", "Coffee-Mug", "coffee mug", "coffee_mug", "-mug", "mug-", "coffee--mug", "café"] {
            let input = NewProduct { handle: handle.to_string(), ..laptop() };

            let error = catalog().new_product(input).unwrap_err();

            assert!(matches!(error, CatalogError::InvalidHandle), "{handle:?} should be rejected");
        }
    }
}
