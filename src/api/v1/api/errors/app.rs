use crate::api::v1::api::errors::utils::traits::{DynReturnableError, ReturnableError};
use axum::{
    response::{IntoResponse, Response},
    Json,
};

#[derive(Debug)]
pub struct AppError(Box<dyn ReturnableError>);

impl<E: ReturnableError> From<E> for AppError {
    fn from(err: E) -> Self {
        AppError(Box::new(err))
    }
}

impl From<DynReturnableError> for AppError {
    fn from(err: DynReturnableError) -> Self {
        AppError(err)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let specs = self.0.specs();
        let body = self.0.body();
        (specs.status_code, Json(body)).into_response()
    }
}
