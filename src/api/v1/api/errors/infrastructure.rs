use axum::http::StatusCode;
use std::any::type_name;
use std::fmt;
use tracing::error;

use crate::api::v1::api::errors::utils::{
    constants::{INFRASTRUCTURE_ERROR_CODE, INFRASTRUCTURE_ERROR_TYPE},
    traits::{ErrorSpecs, ReturnableError},
};

#[derive(Debug)]
pub enum InfrastructureError {
    InternalServerError(String),
    NotFound,
    InvalidData(String), // Represents invalid data that found its way in the db
}

impl ReturnableError for InfrastructureError {
    fn error_type(&self) -> &'static str {
        INFRASTRUCTURE_ERROR_TYPE
    }

    fn base_code(&self) -> u32 {
        INFRASTRUCTURE_ERROR_CODE
    }

    fn specs(&self) -> ErrorSpecs {
        match self {
            InfrastructureError::InternalServerError(_) => ErrorSpecs {
                offset: 10,
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                message: "Internal server error".to_string(),
            },
            InfrastructureError::NotFound => ErrorSpecs {
                offset: 20,
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                message: "Resource not found in database".to_string(),
            },
            InfrastructureError::InvalidData(msg) => ErrorSpecs {
                offset: 30,
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                message: format!("Invalid data: {}", msg),
            },
        }
    }
}

impl fmt::Display for InfrastructureError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message())
    }
}

pub trait IsInfraError: std::fmt::Debug {
    fn as_infra_error(&self) -> InfrastructureError;

    fn log_source_error(&self) {
        let error_type = type_name::<Self>();
        error!("Error [{}]: {:?}", error_type, &self);
    }
}

impl IsInfraError for diesel::result::Error {
    fn as_infra_error(&self) -> InfrastructureError {
        self.log_source_error();
        match self {
            // Map NotFound to InfraError::NotFound
            diesel::result::Error::NotFound => InfrastructureError::NotFound,
            // Map other errors to InfraError::InternalServerError
            _ => InfrastructureError::InternalServerError("diesel result".into()),
        }
    }
}

impl IsInfraError for deadpool_diesel::PoolError {
    fn as_infra_error(&self) -> InfrastructureError {
        self.log_source_error();
        InfrastructureError::InternalServerError("deadpool_diesel pool".into())
    }
}

impl IsInfraError for deadpool_diesel::InteractError {
    fn as_infra_error(&self) -> InfrastructureError {
        self.log_source_error();
        InfrastructureError::InternalServerError("deadpool_diesel interact".into())
    }
}
