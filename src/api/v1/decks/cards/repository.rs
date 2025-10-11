use deadpool_diesel::postgres::Pool;
use diesel::prelude::*;
use diesel::result::Error as DieselError;
use diesel::{PgConnection, QueryDsl};
use uuid::Uuid;

use crate::api::v1::decks::cards::models::db::Card;
use crate::api::v1::decks::cards::models::domain::CardData;
use crate::{
    api::v1::{
        api::errors::infrastructure::InfrastructureError, decks::repository::DeckRepository,
        infra::database::database_interact,
    },
    schema::cards,
};

// const POSITION_OFFSET: i32 = 1000; // TODO: should be global parameter

pub struct CardRepository {}

impl CardRepository {
    pub async fn get_by_id(pool: &Pool, card_id: Uuid) -> Result<Card, InfrastructureError> {
        database_interact(pool, move |conn| Self::sq_get_by_id(conn, card_id)).await
    }

    pub async fn get_cards_by_deck_id(
        pool: &Pool,
        deck_id: Uuid,
    ) -> Result<Vec<Card>, InfrastructureError> {
        database_interact(pool, move |conn| {
            Self::sq_get_cards_by_deck_id(conn, deck_id)
        })
        .await
    }

    pub async fn update_card(
        pool: &Pool,
        card_id: Uuid,
        card_data: CardData,
    ) -> Result<Card, InfrastructureError> {
        database_interact(pool, move |conn| {
            Self::sq_update_card(conn, card_id, card_data)
        })
        .await
    }

    pub async fn insert_new_card(
        pool: &Pool,
        user_id: Uuid,
        deck_id: Uuid,
        card_data: CardData,
    ) -> Result<Card, InfrastructureError> {
        database_interact(pool, move |conn| {
            Self::sq_insert_card(conn, user_id, deck_id, card_data)
        })
        .await
    }
}

// Sub queries
impl CardRepository {
    fn sq_get_by_id(conn: &mut PgConnection, card_id: Uuid) -> Result<Card, DieselError> {
        cards::table
            .filter(cards::id.eq(card_id))
            .first::<Card>(conn)
    }

    pub fn sq_get_cards_by_deck_id(
        conn: &mut PgConnection,
        deck_id: Uuid,
    ) -> Result<Vec<Card>, DieselError> {
        cards::table
            .filter(cards::deck_id.eq(deck_id))
            .filter(cards::is_deleted.eq(false))
            .select(Card::as_select())
            .load(conn)
    }

    fn sq_insert_card(
        conn: &mut PgConnection,
        user_id: Uuid,
        deck_id: Uuid,
        card_data: CardData,
    ) -> Result<Card, DieselError> {
        diesel::insert_into(cards::table)
            .values((
                cards::deck_id.eq(deck_id),
                (cards::created_by.eq(user_id), cards::updated_by.eq(user_id)),
                &card_data,
            ))
            .get_result(conn)
    }

    fn sq_update_card(
        conn: &mut PgConnection,
        card_id: Uuid,
        card_data: CardData,
    ) -> Result<Card, DieselError> {
        diesel::update(cards::table.filter(cards::id.eq(card_id)))
            .set(&card_data)
            .get_result::<Card>(conn)
    }

    pub fn sq_find_position(
        conn: &mut PgConnection,
        deck_sid: String,
        card_id: Uuid,
    ) -> Result<i32, DieselError> {
        let deck_id = DeckRepository::sq_get_id_by_sid(conn, deck_sid)?;
        cards::table
            .filter(cards::deck_id.eq(deck_id).and(cards::id.eq(card_id)))
            .select(cards::position)
            .first::<i32>(conn)
    }

    pub fn sq_get_max_position(
        conn: &mut PgConnection,
        deck_sid: String,
    ) -> Result<i32, DieselError> {
        let deck_id = DeckRepository::sq_get_id_by_sid(conn, deck_sid)?;
        let max_pos: Option<i32> = cards::table
            .filter(cards::deck_id.eq(deck_id))
            .select(diesel::dsl::max(cards::position))
            .first::<Option<i32>>(conn)?;

        Ok(max_pos.unwrap_or(0))
    }
}
