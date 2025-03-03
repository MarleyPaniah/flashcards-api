use axum::{routing::get, Router};

use crate::api::v1::api::state::AppState;

use super::handlers::get_user;

pub fn user_router() -> Router<AppState> {
    let router = Router::new()
        .route("/", get(|| async {}))
        .route("/{user_id}", get(get_user));

    Router::new().nest("/users", router)
}
