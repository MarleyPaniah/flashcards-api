use axum::http::StatusCode;

use crate::api::v1::api::errors::utils::{
    constants::{INPUT_ERROR_CODE, INPUT_ERROR_TYPE},
    traits::{ErrorSpecs, ReturnableError},
};

#[derive(Debug)]
pub enum InputError {
    Bad(String),
    Invalid(String),
}

impl ReturnableError for InputError {
    fn error_type(&self) -> &'static str {
        INPUT_ERROR_TYPE
    }

    fn base_code(&self) -> u32 {
        INPUT_ERROR_CODE
    }

    fn specs(&self) -> ErrorSpecs {
        match self {
            InputError::Bad(msg) => ErrorSpecs {
                offset: 10,
                status_code: StatusCode::BAD_REQUEST,
                message: format!("Bad input: {}", msg),
            },
            InputError::Invalid(msg) => ErrorSpecs {
                offset: 20,
                status_code: StatusCode::UNPROCESSABLE_ENTITY,
                message: format!("Invalid input: {}", msg),
            },
        }
    }
}
