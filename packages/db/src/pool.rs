use std::time::Duration;

use db_generated::deadpool_postgres::{self, Manager, ManagerConfig, RecyclingMethod, Runtime, Timeouts};
use db_generated::tokio_postgres::{self, NoTls};

use crate::DbError;

pub use deadpool_postgres::Pool;

#[derive(Clone, Copy, Debug)]
pub struct PoolSettings {
    pub max_size: usize,
    pub timeout: Duration,
}

pub async fn connect(database_url: &str, settings: PoolSettings) -> Result<Pool, DbError> {
    let config: tokio_postgres::Config = database_url
        .parse()
        .map_err(|error| DbError::InvalidConfig(Box::new(error)))?;

    connect_with(config, settings).await
}

pub(crate) async fn connect_with(
    config: tokio_postgres::Config,
    settings: PoolSettings,
) -> Result<Pool, DbError> {
    let manager = Manager::from_config(
        config,
        NoTls,
        ManagerConfig { recycling_method: RecyclingMethod::Fast },
    );
    let pool = Pool::builder(manager)
        .max_size(settings.max_size)
        .timeouts(Timeouts {
            wait: Some(settings.timeout),
            create: Some(settings.timeout),
            recycle: Some(settings.timeout),
        })
        .runtime(Runtime::Tokio1)
        .build()
        .map_err(|error| DbError::InvalidConfig(Box::new(error)))?;

    drop(pool.get().await?);
    Ok(pool)
}
