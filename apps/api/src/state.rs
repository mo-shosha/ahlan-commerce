use std::sync::Arc;

use catalog::{ImportJobService, ProductCatalog};
use db::Pool;

use crate::config::Config;
use crate::graphql::{AppSchema, GraphqlContext, build_schema};

#[derive(Clone)]
pub struct AppState {
    pub config: Arc<Config>,
    pub db: Pool,
    pub catalog: Arc<ProductCatalog>,
    pub import_jobs: Arc<ImportJobService>,
    pub schema: AppSchema,
}

impl AppState {
    pub fn new(
        config: Config,
        db: Pool,
        catalog: ProductCatalog,
        import_jobs: ImportJobService,
    ) -> Self {
        let catalog = Arc::new(catalog);
        let schema = build_schema(GraphqlContext {
            db: db.clone(),
            catalog: catalog.clone(),
        });

        Self {
            config: Arc::new(config),
            db,
            catalog,
            import_jobs: Arc::new(import_jobs),
            schema,
        }
    }
}
