use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

use axum::extract::rejection::JsonRejection;

use crate::api::v1::infra::errors::InfraError;

use super::error_utils::{ErrorResponse, ReturnableError};
use super::wrappers::AppJson;

// The kinds of errors the application can hit.
#[derive(Debug)]
pub enum AppError {
    Unauthorized,
    InvalidInputJson(JsonRejection),
    ValidationError(String),
    InfrastructureError(InfraError),
}

impl AppError {
    fn error_details(&self) -> (u32, StatusCode, String) {
        match self {
            // TODO: Expand and document
            AppError::Unauthorized => (1001, StatusCode::UNAUTHORIZED, "Unauthorized.".to_string()),
            AppError::InvalidInputJson(_) => (
                1002,
                StatusCode::BAD_REQUEST,
                "Invalid JSON input.".to_string(),
            ),
            AppError::ValidationError(_) => (
                1003,
                StatusCode::BAD_REQUEST,
                "TODO-ValidationError".to_string(),
            ),
            AppError::InfrastructureError(infra_error) => match infra_error {
                // TODO make infra_error implement status_code to avoid repeating it?
                &InfraError::InternalServerError => (
                    1101,
                    StatusCode::INTERNAL_SERVER_ERROR,
                    infra_error.to_string(),
                ),
                &InfraError::NotFound => (
                    1102,
                    StatusCode::INTERNAL_SERVER_ERROR,
                    infra_error.to_string(),
                ),
            },
        }
    }
}

// Implement the ReturnableError trait to AppError to return
// error as JSON
impl ReturnableError for AppError {
    fn error_code(&self) -> u32 {
        self.error_details().0
    }

    fn status_code(&self) -> StatusCode {
        self.error_details().1
    }

    fn message(&self) -> String {
        self.error_details().2
    }
}

// Implement IntoResponse to be able to return it as a response
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (
            self.status_code(),
            AppJson(ErrorResponse {
                message: self.message(),
                error_code: Some(self.error_code()),
            }),
        )
            .into_response()
    }
}
