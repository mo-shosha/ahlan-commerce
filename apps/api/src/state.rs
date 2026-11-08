use std::sync::{Arc, Mutex};

use catalog::ProductCatalog;

use crate::config::Config;

#[derive(Clone)]
pub struct AppState {
    pub config: Arc<Config>,
    pub catalog: Arc<Mutex<ProductCatalog>>,
}

impl AppState {
    pub fn new(config: Config, catalog: ProductCatalog) -> Self {
        Self {
            config: Arc::new(config),
            catalog: Arc::new(Mutex::new(catalog)),
        }
    }
}
