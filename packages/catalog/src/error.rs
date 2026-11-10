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
    Storage(#[from] sqlx::Error),
}
