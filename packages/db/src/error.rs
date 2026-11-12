use std::error::Error as StdError;

use db_generated::{deadpool_postgres, tokio_postgres};

type BoxError = Box<dyn StdError + Send + Sync>;

#[derive(Debug, thiserror::Error)]
pub enum DbError {
    #[error("database configuration is invalid")]
    InvalidConfig(#[source] BoxError),

    #[error("database is unavailable")]
    Unavailable(#[source] BoxError),

    #[error("another product already uses this handle")]
    DuplicateProductHandle,

    #[error("database query failed")]
    Query(#[source] tokio_postgres::Error),
}

impl From<deadpool_postgres::PoolError> for DbError {
    fn from(error: deadpool_postgres::PoolError) -> Self {
        Self::Unavailable(Box::new(error))
    }
}

impl From<tokio_postgres::Error> for DbError {
    fn from(error: tokio_postgres::Error) -> Self {
        let lost_connection = error.is_closed()
            || error.source().is_some_and(|source| source.is::<std::io::Error>());

        if lost_connection {
            Self::Unavailable(Box::new(error))
        } else {
            Self::Query(error)
        }
    }
}
