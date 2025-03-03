use super::errors::{InfraError, IsInfraError};
use std::any::type_name;
use tracing::{error, warn};

// Utility function to adapt errors of generic type T into InfraError
pub fn adapt_infra_error<T: IsInfraError>(error: T) -> InfraError {
    error.as_infra_error()
}

// Helper function for logging and converting errors
pub fn log_and_convert<T: std::fmt::Debug>(error: &T, mapped_error: InfraError) -> InfraError {
    let error_type = type_name::<T>(); // Get the type of the error
    match mapped_error {
        InfraError::NotFound => warn!("Warning [{}]: {:?}", error_type, error),
        _ => error!("Error [{}]: {:?}", error_type, error),
    }
    mapped_error
}
