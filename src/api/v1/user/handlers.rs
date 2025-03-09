use axum::extract::{Path, State};
use axum::{http::StatusCode, response::IntoResponse, Json};

use crate::api::v1::api::state::AppState;

use super::repository::{UserField, UserRepository};

pub async fn get_user(
    State(state): State<AppState>,
    Path(username): Path<String>,
) -> impl IntoResponse {
    let user = UserRepository::select_user(&state.pool, &username, UserField::Username)
        .await
        .unwrap();

    (StatusCode::OK, Json(user));
}

pub async fn delete_user() -> impl IntoResponse {
    (StatusCode::OK, "User deleted.");
}
