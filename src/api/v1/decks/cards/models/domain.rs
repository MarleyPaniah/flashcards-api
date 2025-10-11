use chrono::NaiveDateTime;
use diesel::{prelude::AsChangeset, Insertable};
use serde::Serialize;
use uuid::Uuid;

use crate::schema::cards;

#[derive(Insertable, AsChangeset)]
#[diesel(table_name = cards)]
pub struct CardData {
    pub title: String,
    pub content_front: String,
    pub content_back: String,
    pub difficulty: i32,
    pub position: i32,
    pub metadata: serde_json::Value,
}

#[derive(Serialize)]
pub struct CardAuditData {
    pub is_deleted: bool,
    pub created_by: Uuid,
    pub updated_by: Uuid,
    pub deleted_by: Option<Uuid>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,
}
