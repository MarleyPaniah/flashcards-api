use tracing::trace;

use crate::api::v1::{
    api::{errors::AppError, state::AppState},
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
            .map_err(|e| AppError::UnknownError(format!("{}", e)))?;

        let insert_deck = InsertDeck {
            short_id: short_id_dto.id,
            title: dto.title,
            metadata: dto.metadata,
            updated_by: dto.user_id,
            created_by: dto.user_id,
        };

        let deck_summary = DeckRepository::insert_new_deck(&state.pool, insert_deck).await?;

        return Ok(deck_summary.into());
    }

    /// Get deck by short_id
    pub async fn get_summary_by_sid(
        state: &AppState,
        deck_sid: String,
    ) -> Result<DeckResponseDto, AppError> {
        let deck_summary = DeckRepository::get_deck_summary_by_sid(&state.pool, deck_sid).await?;
        Ok(deck_summary.into())
    }
}
