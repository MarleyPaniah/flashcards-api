use axum::{
    routing::{get, post},
    Router,
};

use crate::api::v1::api::state::AppState;

use super::handlers::{get_user_by_username, register_user};

pub fn auth_router() -> Router<AppState> {
    let router = Router::new().route("/register", post(register_user));

    Router::new().nest("/auth", router)
}

pub fn user_router() -> Router<AppState> {
    let router = Router::new()
        .route("/", get(|| async {}))
        .route("/{username}", get(get_user_by_username));

    Router::new().nest("/users", router)
}
