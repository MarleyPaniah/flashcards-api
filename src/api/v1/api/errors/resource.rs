use axum::http::StatusCode;

use crate::api::v1::api::errors::utils::{
    constants::{RESOURCE_MANAGEMENT_ERROR_CODE, RESOURCE_MANAGEMENT_ERROR_TYPE},
    traits::{ErrorSpecs, ReturnableError},
};

#[derive(Debug)]
pub enum ResourceManagementError {
    ResourceExists(String),
    ResourceDoesNotExist(String),
    ResourceCreation(String),
}

impl ReturnableError for ResourceManagementError {
    fn error_type(&self) -> &'static str {
        RESOURCE_MANAGEMENT_ERROR_TYPE
    }

    fn base_code(&self) -> u32 {
        RESOURCE_MANAGEMENT_ERROR_CODE
    }

    fn specs(&self) -> ErrorSpecs {
        match self {
            ResourceManagementError::ResourceExists(resource) => ErrorSpecs {
                offset: 10,
                status_code: StatusCode::CONFLICT,
                message: format!("{resource} already exists"),
            },
            ResourceManagementError::ResourceDoesNotExist(resource) => ErrorSpecs {
                offset: 20,
                status_code: StatusCode::NOT_FOUND,
                message: format!("{resource} does not exist"),
            },
            ResourceManagementError::ResourceCreation(resource) => ErrorSpecs {
                offset: 30,
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                message: format!("{resource} could not be created"),
            },
        }
    }
}
