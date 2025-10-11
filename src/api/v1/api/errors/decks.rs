use crate::api::v1::api::errors::{
    resource::ResourceManagementError,
    utils::{
        constants::{DECK_ERROR_CODE, DECK_ERROR_TYPE},
        traits::{ErrorSpecs, ReturnableError},
    },
};

#[derive(Debug)]
pub enum DeckError {
    DeckDoesNotExist(String),
    DeckCreation(String),
}

impl ReturnableError for DeckError {
    fn error_type(&self) -> &'static str {
        DECK_ERROR_TYPE
    }

    fn base_code(&self) -> u32 {
        DECK_ERROR_CODE
    }

    fn specs(&self) -> ErrorSpecs {
        match self {
            DeckError::DeckDoesNotExist(deck) => {
                ResourceManagementError::ResourceDoesNotExist(format!("deck '{deck}'")).specs()
            }
            DeckError::DeckCreation(deck) => {
                ResourceManagementError::ResourceCreation(format!("deck '{deck}'")).specs()
            }
        }
    }
}
