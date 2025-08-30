use deadpool_diesel::postgres::Pool;
use diesel::prelude::*;
use diesel::result::Error as DieselError;
use uuid::Uuid;

use crate::{
    api::v1::{
        api::errors::infrastructure::InfrastructureError,
        decks::models::db::{DeckSummary, DeckWithCreator},
        infra::database::database_interact,
    },
    schema::{
        cards,
        decks::{self},
        users,
    },
};

use super::models::db::{Deck, InsertDeck};

pub struct DeckRepository;

impl DeckRepository {
    /// Insert a new deck into the database.
    pub async fn insert_new_deck(
        pool: &Pool,
        new_deck: InsertDeck,
    ) -> Result<DeckSummary, InfrastructureError> {
        let deck_summary = database_interact(pool, |conn| {
            // Insert deck
            let inserted_deck = diesel::insert_into(decks::table)
                .values(new_deck)
                .returning(Deck::as_select())
                .get_result::<Deck>(conn)?;

            let deck_with_creator = Self::sq_get_deck_with_creator_by_id(conn, inserted_deck.id)?;
            let card_count = Self::sq_get_deck_card_count(conn, inserted_deck.id)?;

            Ok(Self::convert_deck_with_creator_to_summary(
                deck_with_creator,
                card_count,
            ))

            // TODO update permission table too
        })
        .await?;

        Ok(deck_summary)
    }

    /// Get a deck summary by its id.
    pub async fn get_deck_summary_by_id(
        pool: &Pool,
        deck_id: Uuid,
    ) -> Result<DeckSummary, InfrastructureError> {
        let deck_summary = database_interact(pool, move |conn| {
            let deck_with_creator = Self::sq_get_deck_with_creator_by_id(conn, deck_id)?;
            let card_count = Self::sq_get_deck_card_count(conn, deck_with_creator.id)?;

            Ok(Self::convert_deck_with_creator_to_summary(
                deck_with_creator,
                card_count,
            ))
        })
        .await?;

        Ok(deck_summary)
    }

    /// Get a deck by its short id.
    pub async fn get_deck_summary_by_sid(
        pool: &Pool,
        deck_sid: String,
    ) -> Result<DeckSummary, InfrastructureError> {
        let deck_summary = database_interact(pool, |conn| {
            let deck_with_creator = Self::sq_get_deck_with_creator_by_sid(conn, deck_sid)?;
            let card_count = Self::sq_get_deck_card_count(conn, deck_with_creator.id)?;

            Ok(Self::convert_deck_with_creator_to_summary(
                deck_with_creator,
                card_count,
            ))
        })
        .await?;

        Ok(deck_summary)
    }

    /// (Synchronous sub query)
    pub fn sq_get_deck_with_creator_by_id(
        conn: &mut PgConnection,
        deck_id: Uuid,
    ) -> Result<DeckWithCreator, DieselError> {
        let deck_with_creator = decks::table
            .inner_join(users::table.on(decks::created_by.eq(users::id)))
            .filter(decks::id.eq(deck_id))
            .filter(decks::is_deleted.eq(false))
            .select((
                decks::id,
                decks::short_id,
                decks::title,
                decks::metadata,
                decks::is_public,
                users::id,
                users::username,
                decks::created_at,
                decks::updated_at,
            ))
            .first::<DeckWithCreator>(conn)?;
        Ok(deck_with_creator)
    }

    /// (Synchronous sub query)
    pub fn sq_get_deck_with_creator_by_sid(
        conn: &mut PgConnection,
        deck_sid: String,
    ) -> Result<DeckWithCreator, DieselError> {
        let deck_with_creator = decks::table
            .inner_join(users::table.on(decks::created_by.eq(users::id)))
            .filter(decks::short_id.eq(deck_sid))
            .filter(decks::is_deleted.eq(false))
            .select((
                decks::id,
                decks::short_id,
                decks::title,
                decks::metadata,
                decks::is_public,
                users::id,
                users::username,
                decks::created_at,
                decks::updated_at,
            ))
            .first::<DeckWithCreator>(conn)?;
        Ok(deck_with_creator)
    }

    /// (Synchronous sub query)
    /// Return the number of cards for a deck given its id.
    pub fn sq_get_deck_card_count(
        conn: &mut PgConnection,
        deck_id: Uuid,
    ) -> Result<u32, DieselError> {
        let _card_count = cards::table
            .filter(cards::deck_id.eq(deck_id))
            .filter(cards::is_deleted.eq(false))
            .count()
            .get_result::<i64>(conn)?;

        if _card_count < 0 || _card_count > u32::MAX as i64 {
            return Err(DieselError::DeserializationError(
                "Card count overflow or negative value".into(),
            ));
        } else {
            return Ok(_card_count as u32);
        };
    }

    pub fn convert_deck_with_creator_to_summary(
        deck_with_creator: DeckWithCreator,
        card_count: u32,
    ) -> DeckSummary {
        return DeckSummary {
            short_id: deck_with_creator.short_id,
            title: deck_with_creator.title,
            metadata: deck_with_creator.metadata,
            is_public: deck_with_creator.is_public,
            card_count: card_count,
            created_by_id: deck_with_creator.created_by_id,
            created_by_name: deck_with_creator.created_by_name,
            created_at: deck_with_creator.created_at,
            updated_at: deck_with_creator.updated_at,
        };
    }
}
