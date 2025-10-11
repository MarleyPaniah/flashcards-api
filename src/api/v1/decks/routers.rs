use axum::{
    routing::{get, post},
    Router,
};

use crate::api::v1::{
    api::state::AppState,
    decks::{
        cards::routers::card_router,
        handlers::{create_deck, get_summary_by_sid},
    },
};

pub fn deck_router() -> Router<AppState> {
    let router = Router::new()
        .route("/", post(create_deck))
        .nest("/{deck_sid}", deck_sid_router());
    Router::new().nest("/decks", router)
}

/**
 * Router after the deck short id (/decks/{deck_sid})
 */
fn deck_sid_router() -> Router<AppState> {
    Router::new()
        .route("/", get(get_summary_by_sid))
        .merge(card_router())
}
