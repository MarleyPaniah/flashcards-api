use chrono::NaiveDateTime;
use diesel::prelude::{Queryable, Selectable};
use uuid::Uuid;

use crate::schema::cards;

#[derive(Queryable, Selectable)]
#[diesel(table_name = cards)]
pub struct Card {
    pub id: Uuid,
    pub deck_id: Uuid,
    pub title: String,
    pub position: i32,
    pub content_front: String,
    pub content_back: String,
    pub difficulty: i32,
    pub metadata: serde_json::Value,
    pub is_deleted: bool,
    pub created_by: Uuid,
    pub updated_by: Uuid,
    pub deleted_by: Option<Uuid>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,
}
