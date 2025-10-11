use crate::api::v1::api::errors::{
    resource::ResourceManagementError,
    utils::{
        constants::{CARD_ERROR_CODE, CARD_ERROR_TYPE},
        traits::{ErrorSpecs, ReturnableError},
    },
};

#[derive(Debug)]
pub enum CardError {
    CardDoesNotExist(String),
    CardCreation(String),
}

impl ReturnableError for CardError {
    fn error_type(&self) -> &'static str {
        CARD_ERROR_TYPE
    }

    fn base_code(&self) -> u32 {
        CARD_ERROR_CODE
    }

    fn specs(&self) -> ErrorSpecs {
        match self {
            CardError::CardDoesNotExist(deck) => {
                ResourceManagementError::ResourceDoesNotExist(format!("card '{deck}'")).specs()
            }
            CardError::CardCreation(deck) => {
                ResourceManagementError::ResourceCreation(format!("card '{deck}'")).specs()
            }
        }
    }
}
