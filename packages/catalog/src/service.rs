use std::sync::Arc;

use sqlx::PgPool;

use crate::{Clock, IdGenerator, NewProduct, Product, CatalogError, store};


pub struct ProductCatalog {
    ids: Arc<dyn IdGenerator>,
    clock: Arc<dyn Clock>,
}

impl ProductCatalog {
    pub fn new(ids: Arc<dyn IdGenerator>, clock: Arc<dyn Clock>) -> Self {
        Self { ids, clock }
    }


    pub async fn create_product(&self, db: &PgPool, input: NewProduct) -> Result<Product, CatalogError> {
        let product = self.new_product(input)?;

        store::insert_product(db, &product).await
    }


    pub async fn list_products(&self, db: &PgPool) -> Result<Vec<Product>, CatalogError> {
        store::list_products(db).await
    }

    fn new_product(&self, input: NewProduct) -> Result<Product, CatalogError> {
        if input.title.trim().is_empty() {
            return Err(CatalogError::ValidationFailed);
        }

        Ok(Product::create(
            self.ids.new_product_id(),
            input,
            self.clock.now(),
        ))
    }
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
            price_cents: 50_000,
            inventory_quantity: 10,
            published: true,
        }
    }

    fn phone() -> NewProduct {
        NewProduct {
            title: "Phone".to_string(),
            handle: "phone".to_string(),
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
}
