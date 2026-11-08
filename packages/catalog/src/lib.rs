mod clock;
mod id;
mod product;
mod service;

#[cfg(any(test, feature = "test-support"))]
pub mod testing;

pub use clock::{Clock, SystemClock, Timestamp};
pub use id::{IdGenerator, ProductId, UuidV7Generator};
pub use product::{NewProduct, Product};
pub use service::ProductCatalog;
