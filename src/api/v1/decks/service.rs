use tracing::trace;

use crate::api::v1::{
    api::{
        errors::{
            app::AppError,
            basic::BasicError,
            decks::DeckError,
            infrastructure::InfrastructureError,
            utils::traits::{DynReturnableError, IntoBoxedError},
        },
        state::AppState,
    },
    decks::{
        models::{db::InsertDeck, requests::NewDeckRequestDto, responses::DeckResponseDto},
        repository::DeckRepository,
    },
    services::short_id::service::ShortIdGeneratorService,
};

pub struct DeckService {}

impl DeckService {
    pub async fn create_deck(
        state: &AppState,
        dto: NewDeckRequestDto,
    ) -> Result<DeckResponseDto, AppError> {
        trace!("Received request to create deck '{}'", dto.title);

        let short_id_dto = ShortIdGeneratorService::generate(state)
            .await
            .map_err(|err| BasicError::UnknownError(err.to_string()))?;

        let _title = dto.title.clone();

        let insert_deck = InsertDeck {
            short_id: short_id_dto.id,
            title: dto.title,
            metadata: dto.metadata,
            updated_by: dto.user_id,
            created_by: dto.user_id,
        };

        let deck_summary = DeckRepository::insert_new_deck(&state.pool, insert_deck)
            .await
            .map_err(|_| DeckError::DeckCreation(_title))?;

        return Ok(deck_summary.into());
    }

    pub async fn get_summary_by_sid(
        state: &AppState,
        deck_sid: String,
    ) -> Result<DeckResponseDto, DynReturnableError> {
        let deck_summary = DeckRepository::get_deck_summary_by_sid(&state.pool, deck_sid.clone())
            .await
            .map_err(|err| Self::adapt_infra_error_deck_info(err, deck_sid))?;

        Ok(deck_summary.into())
    }
}

// Error conversions
impl DeckService {
    pub fn adapt_infra_error_any(_err: InfrastructureError) -> DynReturnableError {
        BasicError::UnknownError(
            "Error occurred when interacting with the deck datasource".to_string(),
        )
        .boxed()
    }

    pub fn adapt_infra_error_deck_info(
        err: InfrastructureError,
        title_or_sid: String,
    ) -> DynReturnableError {
        match err {
            InfrastructureError::NotFound => DeckError::DeckDoesNotExists(title_or_sid).boxed(),
            _ => BasicError::UnknownError(
                "Error while fetching deck by title or short ID".to_string(),
            )
            .boxed(),
        }
    }
}
