use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum CatalogError {
    #[error("Product title is required")]
    ValidationFailed,

    #[error("Another product already uses this handle")]
    DuplicateProductHandle,
}