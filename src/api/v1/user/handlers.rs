use axum::extract::{Path, State};
use axum::{http::StatusCode, response::IntoResponse};

use crate::api::v1::api::errors::AppError;
use crate::api::v1::api::state::AppState;
use crate::api::v1::api::wrappers::AppJson;

use super::models::User;
use super::repository::{UserField, UserRepository};

pub async fn get_user(
    State(state): State<AppState>,
    Path(username): Path<String>,
) -> Result<AppJson<User>, AppError> {
    let user = UserRepository::select_user(&state.pool, &username, UserField::Username)
        .await
        .map_err(|_| {
            AppError::UserDoesNotExist(format!("Username '{}' does not exist.", username))
        })?;

    Ok(AppJson(user))
}

pub async fn delete_user() -> impl IntoResponse {
    (StatusCode::OK, "User deleted.");
}
