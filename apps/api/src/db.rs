use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;

use crate::constants::defaults;

pub fn pool_options() -> PgPoolOptions {
    PgPoolOptions::new()
        .max_connections(defaults::DB_MAX_CONNECTIONS)
        .acquire_timeout(defaults::DB_ACQUIRE_TIMEOUT)
}

pub async fn connect(database_url: &str) -> Result<PgPool, sqlx::Error> {
    pool_options().connect(database_url).await
}
