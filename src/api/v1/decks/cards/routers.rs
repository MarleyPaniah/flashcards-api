use axum::{
    routing::{get, post},
    Router,
};

use crate::api::v1::{
    api::state::AppState,
    decks::cards::handlers::{get_cards, upsert_card},
};

pub fn card_router() -> Router<AppState> {
    let router = Router::new()
        .route("/", get(get_cards))
        .route("/", post(upsert_card));
    Router::new().nest("/cards", router)
}
