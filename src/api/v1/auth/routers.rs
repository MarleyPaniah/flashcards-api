use axum::{routing::post, Router};

use crate::api::v1::api::state::AppState;

use super::handlers::register_user;

pub fn auth_router() -> Router<AppState> {
    let router = Router::new().route("/register", post(register_user));

    Router::new().nest("/auth", router)
}
