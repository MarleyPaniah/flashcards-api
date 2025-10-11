use axum::{
    extract::{Path, State},
    Json,
};

use crate::api::v1::{
    api::{errors::app::AppError, state::AppState, wrappers::AppJsonResponse},
    decks::cards::{
        models::{domain::CardData, requests::UpsertCardRequestDto, responses::CardResponseDto},
        service::CardService,
    },
};

pub async fn get_cards(
    State(state): State<AppState>,
    Path(deck_sid): Path<String>,
) -> Result<AppJsonResponse<Vec<CardResponseDto>>, AppError> {
    let cards = CardService::get_cards_by_deck_sid(&state, deck_sid)
        .await?
        .into_iter()
        .map(Into::into)
        .collect();
    Ok(AppJsonResponse(cards))
}

pub async fn upsert_card(
    State(state): State<AppState>,
    Path(deck_sid): Path<String>,
    Json(payload): Json<UpsertCardRequestDto>,
) -> Result<AppJsonResponse<CardResponseDto>, AppError> {
    let user_id = payload.user_id; // TODO: temporary, until user resolution is implemented
    let card_id = payload.id;
    let card_data: CardData = payload.into();
    let card = CardService::upsert_card(&state, user_id, deck_sid, card_id, card_data)
        .await?
        .into();
    Ok(AppJsonResponse(card))
}
