use thiserror::Error;

#[derive(Debug, Error)]
pub enum CatalogError {
    #[error("Product title is required")]
    ValidationFailed,

    #[error("Handle must use lowercase letters, digits, and single hyphens")]
    InvalidHandle,

    #[error("{field} must be at most {max}")]
    ValueTooLarge { field: &'static str, max: i32 },

    #[error("Another product already uses this handle")]
    DuplicateProductHandle,

    #[error("Product storage failed")]
    Storage(#[source] db::DbError),

    #[error("Stored {field} is out of range: {value}")]
    InvalidStoredValue { field: &'static str, value: i32 },

    #[error("Product not found")]
    ProductNotFound,

    #[error("Product ID is invalid")]
    InvalidProductId,
}

impl From<db::DbError> for CatalogError {
    fn from(error: db::DbError) -> Self {
        match error {
            db::DbError::DuplicateProductHandle => Self::DuplicateProductHandle,
            other => Self::Storage(other),
        }
    }
}
