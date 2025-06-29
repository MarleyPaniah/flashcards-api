use axum::{
    extract::{Path, State},
    Json,
};

use crate::api::v1::{
    api::{errors::AppError, state::AppState, wrappers::AppJson},
    decks::service::DeckService,
};

use super::models::requests::NewDeckRequestDto;
use super::models::responses::DeckResponseDto;

pub async fn create_deck(
    state: State<AppState>,
    Json(payload): Json<NewDeckRequestDto>,
) -> Result<AppJson<DeckResponseDto>, AppError> {
    let deck = DeckService::create_deck(&state, payload).await?;
    return Ok(AppJson(deck));
}

pub async fn get_summary_by_sid(
    State(state): State<AppState>,
    Path(deck_sid): Path<String>,
) -> Result<AppJson<DeckResponseDto>, AppError> {
    let deck_summary = DeckService::get_summary_by_sid(&state, deck_sid).await?;
    Ok(AppJson(deck_summary))
}
