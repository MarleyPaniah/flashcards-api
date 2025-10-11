use crate::api::v1::api::errors::{
    resource::ResourceManagementError,
    utils::{
        constants::{USER_ERROR_CODE, USER_ERROR_TYPE},
        traits::{ErrorSpecs, ReturnableError},
    },
};

#[derive(Debug)]
pub enum UserError {
    UserExists(String),
    UserDoesNotExist(String),
    EmailInUse(String),
}

impl ReturnableError for UserError {
    fn error_type(&self) -> &'static str {
        USER_ERROR_TYPE
    }

    fn base_code(&self) -> u32 {
        USER_ERROR_CODE
    }

    fn specs(&self) -> ErrorSpecs {
        match self {
            UserError::UserExists(user) => {
                ResourceManagementError::ResourceExists(format!("user '{user}'")).specs()
            }

            UserError::UserDoesNotExist(user) => {
                ResourceManagementError::ResourceDoesNotExist(format!("user '{user}'")).specs()
            }

            UserError::EmailInUse(email) => {
                ResourceManagementError::ResourceExists(format!("email '{email}'")).specs()
            }
        }
    }
}
