use std::sync::Arc;

use api::{config::Config, router, state::AppState};
use catalog::{ProductCatalog, SystemClock, UuidV7Generator};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let config = Config::from_env().unwrap_or_else(|err| panic!("invalid configuration: {err}"));
    let addr = config.api_addr;

    let catalog = ProductCatalog::new(Arc::new(UuidV7Generator), Arc::new(SystemClock));
    let state = AppState::new(config, catalog);

    let listener = TcpListener::bind(addr)
        .await
        .unwrap_or_else(|err| panic!("failed to bind {addr}: {err}"));

    println!("Ahlan Commerce API listening on {addr}");

    axum::serve(listener, router(state))
        .await
        .expect("server failed");
}
