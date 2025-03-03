// ERROR TRAIT
//

use serde::Serialize;

use axum::http::StatusCode;

// Define a custom ReturnableError trait, to return some errors
// (even internal) in the form of JSON.
pub trait ReturnableError: std::fmt::Debug {
    fn error_code(&self) -> u32;
    fn status_code(&self) -> StatusCode;
    fn message(&self) -> String;
}

// How we want errors responses to be serialized
#[derive(Serialize)]
pub struct ErrorResponse {
    pub message: String,
    pub error_code: Option<u32>,
}
