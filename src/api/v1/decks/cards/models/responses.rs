use serde::Serialize;
use uuid::Uuid;

use crate::api::v1::decks::cards::models::domain::CardAuditData;

#[derive(Serialize)]
pub struct CardResponseDto {
    pub id: Uuid,
    pub deck_id: Uuid,
    pub title: String,
    pub position: i32,
    pub content_front: String,
    pub content_back: String,
    pub difficulty: i32,
    pub metadata: serde_json::Value,
    pub audit_data: CardAuditData,
}
