use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct UpsertCardRequestDto {
    pub id: Option<Uuid>,
    pub user_id: Uuid, // TODO: temporary, until user resolution is implemented
    pub title: String,
    pub content_front: String,
    pub content_back: String,
    pub difficulty: i32,
    pub position: Option<i32>,
    pub metadata: serde_json::Value,
}
