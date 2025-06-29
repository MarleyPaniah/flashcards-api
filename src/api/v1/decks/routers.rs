use axum::{
    routing::{get, post},
    Router,
};

use crate::api::v1::{
    api::state::AppState,
    decks::handlers::{create_deck, get_summary_by_sid},
};

pub fn deck_router() -> Router<AppState> {
    let router = Router::new()
        .route("/", post(create_deck))
        .route("/{deck_sid}", get(get_summary_by_sid));

    Router::new().nest("/decks", router)
}
