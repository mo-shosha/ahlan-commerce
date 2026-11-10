mod clock;
mod id;
mod product;
mod service;
mod store;
pub mod error;

pub use error::CatalogError;

#[cfg(any(test, feature = "test-support"))]
pub mod testing;

pub use clock::{Clock, SystemClock, Timestamp};
pub use id::{IdGenerator, ProductId, UuidV7Generator};
pub use product::{NewProduct, Product};
pub use service::ProductCatalog;

