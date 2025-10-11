use uuid::Uuid;

use crate::api::v1::{
    api::{
        errors::{
            basic::BasicError,
            cards::CardError,
            decks::DeckError,
            infrastructure::InfrastructureError,
            utils::traits::{DynReturnableError, IntoBoxedError},
        },
        state::AppState,
    },
    decks::{
        cards::{
            models::{db::Card, domain::CardData},
            repository::CardRepository,
        },
        service::DeckService,
    },
};

pub struct CardService {}

impl CardService {
    pub async fn get_by_id(state: &AppState, card_id: Uuid) -> Result<Card, DynReturnableError> {
        CardRepository::get_by_id(&state.pool, card_id)
            .await
            .map_err(|err| Self::adapt_infra_error_get(err, card_id.to_string()))
    }

    pub async fn get_cards_by_deck_sid(
        state: &AppState,
        deck_sid: String,
    ) -> Result<Vec<Card>, DynReturnableError> {
        let deck_id = DeckService::get_id_by_sid(&state, deck_sid.clone()).await?;
        CardRepository::get_cards_by_deck_id(&state.pool, deck_id)
            .await
            .map_err(|err| Self::adapt_infra_error_any(err))
    }

    pub async fn upsert_card(
        state: &AppState,
        user_id: Uuid,
        deck_sid: String,
        card_id: Option<Uuid>,
        card_data: CardData,
    ) -> Result<Card, DynReturnableError> {
        let deck_id = DeckService::get_id_by_sid(state, deck_sid).await?;

        match card_id {
            Some(id) => Self::update_card(state, deck_id, id, card_data).await,
            None => CardRepository::insert_new_card(&state.pool, user_id, deck_id, card_data)
                .await
                .map_err(|err| Self::adapt_infra_error_any(err)),
        }
    }

    pub async fn update_card(
        state: &AppState,
        deck_id: Uuid,
        card_id: Uuid,
        card_data: CardData,
    ) -> Result<Card, DynReturnableError> {
        let existing_card = CardService::get_by_id(&state, card_id).await?;

        if existing_card.deck_id != deck_id {
            return Err(DeckError::DeckDoesNotExist(deck_id.to_string()).boxed());
        }

        CardRepository::update_card(&state.pool, card_id, card_data)
            .await
            .map_err(|err| Self::adapt_infra_error_any(err))
    }
}

// Error conversions
impl CardService {
    pub fn adapt_infra_error_any(_err: InfrastructureError) -> DynReturnableError {
        BasicError::UnknownError(format!(
            "Error occurred when interacting with the card datasource: {_err}"
        ))
        .boxed()
    }

    pub fn adapt_infra_error_get(err: InfrastructureError, id: String) -> DynReturnableError {
        match err {
            InfrastructureError::NotFound => CardError::CardDoesNotExist(id).boxed(),
            _ => BasicError::UnknownError("Error while fetching card by id".to_string()).boxed(),
        }
    }
}
