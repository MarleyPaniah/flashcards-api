use axum::{routing::get, Router};

use super::handlers::{error_404_handler, health_checker_handler, ping_handler};
use super::state::AppState;

use crate::api::v1::auth::routers::auth_router;
use crate::api::v1::user::routers::user_router;

pub fn api_router() -> Router<AppState> {
    let router = Router::new()
        .route("/", get(|| async {}))
        .route("/ping", get(ping_handler))
        .route("/health", get(health_checker_handler))
        .merge(auth_router())
        .merge(user_router());

    Router::new()
        .nest("/api/v1", router)
        .fallback(error_404_handler)
}
