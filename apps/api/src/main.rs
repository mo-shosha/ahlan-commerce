use std::sync::Arc;

use api::{config::Config, constants::defaults, db, router, state::AppState};
use catalog::{ProductCatalog, SystemClock, UuidV7Generator};
use tokio::net::TcpListener;
use tracing_subscriber::EnvFilter;

fn init_tracing() {
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info"));

    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .init();
}


#[tokio::main]
async fn main() {

    init_tracing();
    let config = Config::from_env().unwrap_or_else(|err| panic!("invalid configuration: {err}"));
    let addr = config.api_addr;

    let pool = db::connect(&config.database_url)
        .await
        .unwrap_or_else(|err| panic!("failed to connect to PostgreSQL: {err}"));

    tracing::info!(
        max_connections = defaults::DB_MAX_CONNECTIONS,
        "PostgreSQL connection pool ready"
    );

    let catalog = ProductCatalog::new(Arc::new(UuidV7Generator), Arc::new(SystemClock));
    let state = AppState::new(config, pool, catalog);

    let listener = TcpListener::bind(addr)
        .await
        .unwrap_or_else(|err| panic!("failed to bind {addr}: {err}"));

    tracing::info!(addr = %addr, "Ahlan Commerce API listening");

    axum::serve(listener, router(state))
        .await
        .expect("server failed");
}
