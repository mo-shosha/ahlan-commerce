mod error;
mod pool;
pub mod products;

#[cfg(any(test, feature = "test-support"))]
pub mod testing;

pub use error::DbError;
pub use pool::{Pool, PoolSettings, connect};
