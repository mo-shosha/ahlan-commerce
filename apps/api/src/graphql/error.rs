use async_graphql::{Error, ErrorExtensions};
use catalog::CatalogError;

use crate::app_error::{AppError, AppErrorKind};

pub fn from_catalog(error: CatalogError, request_id: &str) -> Error {
    let app_error = AppError::from_catalog(error, request_id.to_string());
    from_app_error(app_error)
}

pub fn validation(message: impl Into<String>, request_id: &str) -> Error {
    from_app_error(AppError::new(
        AppErrorKind::ValidationFailed(message.into()),
        request_id.to_string(),
    ))
}

fn from_app_error(error: AppError) -> Error {
    let code = error.error_code();
    let message = match &error.kind {
        AppErrorKind::ValidationFailed(message) => message.clone(),
        AppErrorKind::DuplicateProductHandle => {
            "Another product already uses this handle.".to_string()
        }
        AppErrorKind::NotFound => "The requested resource does not exist.".to_string(),
        AppErrorKind::DependencyUnavailable => {
            "A required service is temporarily unavailable.".to_string()
        }
        AppErrorKind::InternalError => "An unexpected server error occurred.".to_string(),
    };

    if let Some(cause) = &error.cause {
        tracing::error!(
            request_id = %error.request_id,
            error_code = code,
            cause = %cause,
            "GraphQL request failed"
        );
    } else {
        tracing::warn!(
            request_id = %error.request_id,
            error_code = code,
            "GraphQL request failed"
        );
    }

    let request_id = error.request_id;
    Error::new(message).extend_with(move |_, extensions| {
        extensions.set("code", code);
        extensions.set("request_id", request_id);
    })
}
