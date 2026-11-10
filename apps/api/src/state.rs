use std::sync::Arc;

use catalog::ProductCatalog;
use sqlx::PgPool;

use crate::config::Config;

#[derive(Clone)]
pub struct AppState {
    pub config: Arc<Config>,
    pub db: PgPool,
    pub catalog: Arc<ProductCatalog>,
}

impl AppState {
    pub fn new(config: Config, db: PgPool, catalog: ProductCatalog) -> Self {
        Self {
            config: Arc::new(config),
            db,
            catalog: Arc::new(catalog),
        }
    }
}
