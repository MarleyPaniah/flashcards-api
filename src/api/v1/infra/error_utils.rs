use crate::api::v1::api::errors::infrastructure::{InfrastructureError, IsInfraError};

// Utility function to adapt errors of generic type T into InfraError
pub fn adapt_infra_error<T: IsInfraError>(error: T) -> InfrastructureError {
    error.as_infra_error()
}
