
use axum::{ http::StatusCode,response::IntoResponse,Json};
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

    fn error_code(&self) -> &'static str {
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
            catalog::CatalogError::DuplicateProductHandle => {
                AppErrorKind::DuplicateProductHandle
            }
        };

        Self::new(kind, request_id)
    }

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
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        if let AppErrorKind::InternalError = self.kind {
            eprintln!("{}", self.log_message());
        }

        let status = self.status_code();

        let body = ErrorResponse {
            error: ErrorDetails {
                code: self.error_code(),
                message: self.public_message(),
                request_id: self.request_id,
            },
        };

        (status, Json(body)).into_response()
    }
}


impl From<CatalogError> for AppError {
    fn from(error: CatalogError) -> Self {
        let kind = match error {
            CatalogError::ValidationFailed => {
                AppErrorKind::ValidationFailed(
                    "Product title is required.".to_string(),
                )
            }
            CatalogError::DuplicateProductHandle => {
                AppErrorKind::DuplicateProductHandle
            }
        };

        Self::new(kind, "temporary-request-id".to_string())
    }
}
