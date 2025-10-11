use axum::http::StatusCode;

use crate::api::v1::api::errors::utils::{
    constants::{BASIC_ERROR_CODE, BASIC_ERROR_TYPE},
    traits::{ErrorSpecs, ReturnableError},
};

#[derive(Debug)]
pub enum BasicError {
    Unauthorized,
    Forbidden(String),
    UnknownError(String),
}

impl ReturnableError for BasicError {
    fn error_type(&self) -> &'static str {
        BASIC_ERROR_TYPE
    }

    fn base_code(&self) -> u32 {
        BASIC_ERROR_CODE
    }

    fn specs(&self) -> ErrorSpecs {
        match self {
            BasicError::Unauthorized => ErrorSpecs {
                offset: 10,
                status_code: StatusCode::UNAUTHORIZED,
                message: "Unauthorized access".to_string(),
            },
            BasicError::Forbidden(_) => ErrorSpecs {
                offset: 20,
                status_code: StatusCode::FORBIDDEN,
                message: "Forbidden access".to_string(),
            },
            BasicError::UnknownError(_) => ErrorSpecs {
                offset: 90,
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                message: "Unknown error occurred".to_string(),
            },
        }
    }
}
