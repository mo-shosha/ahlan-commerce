use std::sync::Arc;

use crate::{Clock, IdGenerator, NewProduct, Product, CatalogError};


pub struct ProductCatalog {
    products: Vec<Product>,
    ids: Arc<dyn IdGenerator>,
    clock: Arc<dyn Clock>,
}

impl ProductCatalog {
    pub fn new(ids: Arc<dyn IdGenerator>, clock: Arc<dyn Clock>) -> Self {
        Self {
            products: Vec::new(),
            ids,
            clock,
        }
    }

    
    pub fn create_product(&mut self,input: NewProduct) -> Result<Product, CatalogError> {
        if input.title.trim().is_empty() {
            return Err(CatalogError::ValidationFailed);
        }

        if self.products.iter().any(|product| product.handle == input.handle) {
            return Err(CatalogError::DuplicateProductHandle);
        }

        let product = Product::create(
            self.ids.new_product_id(),
            input,
            self.clock.now(),
        );

        self.products.push(product.clone());

        Ok(product)
    }


    pub fn list_products(&self) -> Vec<Product> {
        self.products.clone()
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
    fn create_product_copies_input_fields() {
        let product = catalog().create_product(laptop()).expect("laptop is valid");

        assert_eq!(product.title, "Laptop");
        assert_eq!(product.handle, "laptop");
        assert_eq!(product.price_cents, 50_000);
        assert_eq!(product.inventory_quantity, 10);
        assert!(product.published);
    }

    #[test]
    fn create_product_assigns_id_from_generator() {
        let product = catalog().create_product(laptop()).expect("laptop is valid");

        assert_eq!(product.id.to_string(), FIRST_ID);
    }

    #[test]
    fn create_product_sets_both_timestamps_from_clock() {
        let product = catalog().create_product(laptop()).expect("laptop is valid");
        let now = FixedClock::from_rfc3339(NOW).now();

        assert_eq!(product.created_at, now);
        assert_eq!(product.updated_at, now);
    }

    #[test]
    fn each_created_product_gets_the_next_id() {
        let mut catalog = catalog();

        let first = catalog.create_product(laptop()).expect("laptop is valid");
        let second = catalog.create_product(phone()).expect("phone is valid");

        assert_eq!(first.id.to_string(), FIRST_ID);
        assert_eq!(second.id.to_string(), SECOND_ID);
    }

    #[test]
    fn list_products_is_empty_for_new_catalog() {
        assert!(catalog().list_products().is_empty());
    }

    #[test]
    fn list_products_returns_products_in_creation_order() {
        let mut catalog = catalog();
        let first = catalog.create_product(laptop()).expect("laptop is valid");
        let second = catalog.create_product(phone()).expect("phone is valid");

        assert_eq!(catalog.list_products(), vec![first, second]);
    }

    #[test]
    fn list_products_does_not_modify_catalog() {
        let mut catalog = catalog();
        catalog.create_product(laptop()).expect("laptop is valid");

        assert_eq!(catalog.list_products(), catalog.list_products());
        assert_eq!(catalog.list_products().len(), 1);
    }
}
