use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct NewDeckRequestDto {
    pub title: String,
    pub user_id: Uuid, // TODO remove when AuthN
    pub metadata: serde_json::Value,
}
