use std::sync::Arc;

use catalog::{ImportJobService, ProductCatalog, SystemClock, UuidV7Generator};
use tracing_subscriber::EnvFilter;
use worker::{Config, Worker, defaults};

fn init_tracing() {
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .init();
}

#[tokio::main]
async fn main() {
    init_tracing();

    let config = Config::from_env().unwrap_or_else(|err| panic!("invalid configuration: {err}"));

    let pool_settings = db::PoolSettings {
        max_size: defaults::DB_MAX_CONNECTIONS,
        timeout: defaults::DB_ACQUIRE_TIMEOUT,
    };
    let pool = db::connect(&config.database_url, pool_settings)
        .await
        .unwrap_or_else(|err| panic!("failed to connect to PostgreSQL: {err}"));

    tracing::info!(
        max_connections = defaults::DB_MAX_CONNECTIONS,
        poll_interval_ms = config.poll_interval.as_millis() as u64,
        "Import worker connected to PostgreSQL"
    );

    let ids = Arc::new(UuidV7Generator);
    let clock = Arc::new(SystemClock);
    let catalog = ProductCatalog::new(ids.clone(), clock.clone());
    let import_jobs = ImportJobService::new(ids, clock);

    let worker = Worker::new(pool, catalog, import_jobs);
    worker.run(config.poll_interval).await;
}
