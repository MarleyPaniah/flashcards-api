use std::fmt;

use crate::api::v1::api::errors::AppError;

use super::error_utils::log_and_convert;

// Define a custom error type for infrastructure-related errors
#[derive(Debug)]
pub enum InfraError {
    InternalServerError, // Represents an internal server error
    NotFound,            // Represents a resource not found error
}

// Implement the Display trait to customize how InfraError is displayed
impl fmt::Display for InfraError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            // Display "Not found" for NotFound variant
            InfraError::NotFound => write!(f, "Not found"),
            // Display "Internal server error" for InternalServerError variant
            InfraError::InternalServerError => write!(f, "Internal server error"),
        }
    }
}

// Define a custom Error trait for types that can be converted to InfraError
pub trait IsInfraError {
    fn as_infra_error(&self) -> InfraError;
}

// Implement the IsInfraError trait for diesel::result::Error
impl IsInfraError for diesel::result::Error {
    fn as_infra_error(&self) -> InfraError {
        let mapped_error = match self {
            // Map NotFound to InfraError::NotFound
            diesel::result::Error::NotFound => InfraError::NotFound,
            // Map other errors to InfraError::InternalServerError
            _ => InfraError::InternalServerError,
        };
        log_and_convert(self, mapped_error)
    }
}

// Implement the IsInfraError trait for deadpool_diesel::PoolError
impl IsInfraError for deadpool_diesel::PoolError {
    fn as_infra_error(&self) -> InfraError {
        // Map all PoolError instances to InfraError::InternalServerError
        log_and_convert(self, InfraError::InternalServerError)
    }
}

// Implement the IsInfraError trait for InteractError
impl IsInfraError for deadpool_diesel::InteractError {
    fn as_infra_error(&self) -> InfraError {
        // Map all InteractError instances to InfraError::InternalServerError
        log_and_convert(self, InfraError::InternalServerError)
    }
}

// Implement From<InfraError> for AppError to be able to use into()
impl From<InfraError> for AppError {
    fn from(err: InfraError) -> Self {
        match err {
            _ => AppError::InfrastructureError(err),
        }
    }
}
