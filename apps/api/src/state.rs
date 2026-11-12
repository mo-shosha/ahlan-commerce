use std::sync::Arc;

use catalog::ProductCatalog;
use db::Pool;

use crate::config::Config;

#[derive(Clone)]
pub struct AppState {
    pub config: Arc<Config>,
    pub db: Pool,
    pub catalog: Arc<ProductCatalog>,
}

impl AppState {
    pub fn new(config: Config, db: Pool, catalog: ProductCatalog) -> Self {
        Self {
            config: Arc::new(config),
            db,
            catalog: Arc::new(catalog),
        }
    }
}
