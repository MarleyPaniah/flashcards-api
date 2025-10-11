use chrono::NaiveDateTime;
use diesel::prelude::{Identifiable, Insertable, Queryable, Selectable};
use serde::Deserialize;
use uuid::Uuid;

use crate::schema::decks;

#[derive(Debug, Clone, Queryable, Selectable, Identifiable)]
#[diesel(table_name = decks)]
pub struct Deck {
    pub id: Uuid,
    pub short_id: String,
    pub title: String,
    pub metadata: serde_json::Value,
    pub is_public: bool,
    pub is_deleted: bool,
    pub created_by: Uuid,
    pub updated_by: Uuid,
    pub deleted_by: Option<Uuid>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,
}

#[derive(Deserialize, Insertable)]
#[diesel(table_name = decks)]
pub struct InsertDeck {
    pub short_id: String,
    pub title: String,
    pub metadata: serde_json::Value,
    pub created_by: Uuid,
    pub updated_by: Uuid,
}

#[derive(Queryable)]
pub struct DeckWithCreator {
    pub id: Uuid,
    pub short_id: String,
    pub title: String,
    pub metadata: serde_json::Value,
    pub is_public: bool,
    pub created_by_id: Uuid,
    pub created_by_name: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Queryable)]
pub struct DeckSummary {
    pub short_id: String,
    pub title: String,
    pub metadata: serde_json::Value,
    pub card_count: u32,
    pub is_public: bool,
    pub created_by_id: Uuid,
    pub created_by_name: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
