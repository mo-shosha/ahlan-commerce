use std::sync::Arc;

use catalog::ProductCatalog;
use db::Pool;

use crate::config::Config;
use crate::graphql::{AppSchema, GraphqlContext, build_schema};

#[derive(Clone)]
pub struct AppState {
    pub config: Arc<Config>,
    pub db: Pool,
    pub catalog: Arc<ProductCatalog>,
    pub schema: AppSchema,
}

impl AppState {
    pub fn new(config: Config, db: Pool, catalog: ProductCatalog) -> Self {
        let catalog = Arc::new(catalog);
        let schema = build_schema(GraphqlContext {
            db: db.clone(),
            catalog: catalog.clone(),
        });

        Self {
            config: Arc::new(config),
            db,
            catalog,
            schema,
        }
    }
}
