
use axum::{ extract::rejection::JsonRejection, http::StatusCode,response::IntoResponse,Json};
use rootcause::Report;
use serde::Serialize;
use catalog::CatalogError;
#[derive(Debug)]
pub enum AppErrorKind {
    ValidationFailed(String),
    DuplicateProductHandle,
    NotFound,
    DependencyUnavailable,
    InternalError,
}

#[derive(Debug)]
pub struct AppError {
    pub kind: AppErrorKind,
    pub request_id: String,
    // Server-side only. Never copied into the HTTP response.
    pub cause: Option<Report>,
}

#[derive(Debug, Serialize)]
struct ErrorResponse {
    error: ErrorDetails,
}

#[derive(Debug, Serialize)]
struct ErrorDetails {
    code: &'static str,
    message: String,
    request_id: String,
}

impl AppError {
    pub fn new(kind: AppErrorKind, request_id: String) -> Self {
        Self { kind, request_id, cause: None }
    }

    pub fn internal(cause: Report, request_id: String) -> Self {
        Self {
            kind: AppErrorKind::InternalError,
            request_id,
            cause: Some(cause),
        }
    }

    fn status_code(&self) -> StatusCode {
        match &self.kind {
            AppErrorKind::ValidationFailed(_) => StatusCode::BAD_REQUEST,
            AppErrorKind::DuplicateProductHandle => StatusCode::CONFLICT,
            AppErrorKind::NotFound => StatusCode::NOT_FOUND,
            AppErrorKind::DependencyUnavailable => {
                StatusCode::SERVICE_UNAVAILABLE
            }
            AppErrorKind::InternalError => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
        }
    }

    pub fn error_code(&self) -> &'static str {
        match &self.kind {
            AppErrorKind::ValidationFailed(_) => "validation_failed",
            AppErrorKind::DuplicateProductHandle => {
                "duplicate_product_handle"
            }
            AppErrorKind::NotFound => "not_found",
            AppErrorKind::DependencyUnavailable => {
                "dependency_unavailable"
            }
            AppErrorKind::InternalError => "internal_error",
        }
    }

    fn public_message(&self) -> String {
        match &self.kind {
            AppErrorKind::ValidationFailed(message) => message.clone(),
            AppErrorKind::DuplicateProductHandle => {
                "Another product already uses this handle.".to_string()
            }
            AppErrorKind::NotFound => {
                "The requested resource does not exist.".to_string()
            }
            AppErrorKind::DependencyUnavailable => {
                "A required service is temporarily unavailable.".to_string()
            }
            AppErrorKind::InternalError => {
                "An unexpected server error occurred.".to_string()
            }
        }
    }


    pub fn from_catalog(error: catalog::CatalogError,request_id: String) -> Self {
        let kind = match error {
            catalog::CatalogError::ValidationFailed => {
                AppErrorKind::ValidationFailed(
                    "Product title is required.".to_string(),
                )
            }
            catalog::CatalogError::InvalidHandle => {
                AppErrorKind::ValidationFailed(
                    "Handle must use lowercase letters, digits, and single hyphens, for example coffee-mug.".to_string(),
                )
            }
            catalog::CatalogError::ValueTooLarge { field, max } => {
                AppErrorKind::ValidationFailed(format!("{field} must be at most {max}."))
            }
            catalog::CatalogError::DuplicateProductHandle => {
                AppErrorKind::DuplicateProductHandle
            }
            catalog::CatalogError::Storage(cause) => {
                return Self::from_storage(cause, request_id);
            }
            error @ catalog::CatalogError::InvalidStoredValue { .. } => {
                return Self::internal(Report::from(error), request_id);
            }

            catalog::CatalogError::ProductNotFound => {
                AppErrorKind::NotFound
            }
            catalog::CatalogError::InvalidProductId => {
                AppErrorKind::ValidationFailed("Product ID is invalid.".to_string())
            }
        };

        Self::new(kind, request_id)
    }

    pub fn from_json_rejection(rejection: JsonRejection, request_id: String) -> Self {
        Self::new(AppErrorKind::ValidationFailed(rejection.body_text()), request_id)
    }

    fn from_storage(cause: db::DbError, request_id: String) -> Self {
        let kind = match &cause {
            db::DbError::Unavailable(_) => AppErrorKind::DependencyUnavailable,
            _ => AppErrorKind::InternalError,
        };

        Self {
            kind,
            request_id,
            cause: Some(Report::from(cause)),
        }
    }
    /* 
    fn log_message(&self) -> String {
        let root_cause = match &self.cause {
            Some(cause) => format!("{cause}"),
            None => "<no cause recorded>".to_string(),
        };

        format!(
            "ERROR request_id={} error_code={} root_cause:\n{}",
            self.request_id,
            self.error_code(),
            root_cause,
        )
    }*/
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let status = self.status_code();
        let error_code = self.error_code();
        tracing::Span::current().record("error_code", error_code);
        
        match &self.cause {
            Some(cause) => {
                tracing::error!(
                    request_id = %self.request_id,
                    error_code,
                    status = status.as_u16(),
                    cause = %cause,
                    "Request failed"
                );
            }
            None => {
                tracing::warn!(
                    request_id = %self.request_id,
                    error_code,
                    status = status.as_u16(),
                    "Request failed"
                );
            }
        }

        let body = ErrorResponse {
            error: ErrorDetails {
                code: error_code,
                message: self.public_message(),
                request_id: self.request_id,
            },
        };

        (status, Json(body)).into_response()
    }
}


impl From<CatalogError> for AppError {
    fn from(error: CatalogError) -> Self {
        Self::from_catalog(error, "temporary-request-id".to_string())
    }
}
