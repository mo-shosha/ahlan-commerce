mod clock;
mod id;
mod import_job;
mod import_service;
mod product;
mod records;
mod service;
pub mod error;

pub use error::CatalogError;

#[cfg(any(test, feature = "test-support"))]
pub mod testing;

pub use clock::{Clock, SystemClock, Timestamp};
pub use id::{IdGenerator, ImportJobId, ProductId, UuidV7Generator};
pub use import_job::{
    ImportJob, MAX_IMPORT_ATTEMPTS, STATUS_FAILED, STATUS_QUEUED, STATUS_RUNNING, STATUS_SUCCEEDED,
};
pub use import_service::ImportJobService;
pub use product::{NewProduct, Product};
pub use service::ProductCatalog;

