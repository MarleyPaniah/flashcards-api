use chrono::NaiveDateTime;
use serde::Serialize;

use crate::api::v1::{decks::models::db::DeckSummary, users::models::responses::UserResponseDto};

#[derive(Serialize)]
pub struct DeckResponseDto {
    pub short_id: String,
    pub title: String,
    pub metadata: serde_json::Value,
    pub card_count: u32,
    pub created_by: UserResponseDto,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl From<DeckSummary> for DeckResponseDto {
    fn from(deck_summary: DeckSummary) -> Self {
        DeckResponseDto {
            short_id: deck_summary.short_id,
            title: deck_summary.title,
            metadata: deck_summary.metadata,
            card_count: deck_summary.card_count,
            created_by: UserResponseDto {
                id: deck_summary.created_by_id,
                username: deck_summary.created_by_name,
            },
            created_at: deck_summary.created_at,
            updated_at: deck_summary.updated_at,
        }
    }
}
