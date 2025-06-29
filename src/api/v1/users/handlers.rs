use axum::extract::{Path, State};
use axum::Json;
use axum::{http::StatusCode, response::IntoResponse};

use crate::api::v1::api::errors::AppError;
use crate::api::v1::api::state::AppState;
use crate::api::v1::api::wrappers::AppJson;
use crate::api::v1::users::models::requests::NewUserRequestDto;
use crate::api::v1::users::service::UserService;

use super::models::responses::UserResponseDto;

pub async fn register_user(
    State(state): State<AppState>,
    Json(payload): Json<NewUserRequestDto>,
) -> Result<AppJson<UserResponseDto>, AppError> {
    let user = UserService::create_user(&state, payload).await?;
    Ok(AppJson(user))
}

pub async fn get_user_by_username(
    State(state): State<AppState>,
    Path(username): Path<String>,
) -> Result<AppJson<UserResponseDto>, AppError> {
    let user = UserService::get_user_by_username(&state, username.clone())
        .await
        .map_err(|_| {
            AppError::UserDoesNotExist(format!("Username '{}' does not exist.", username))
        })?;

    Ok(AppJson(user))
}

pub async fn delete_user() -> impl IntoResponse {
    (StatusCode::OK, "User deleted.");
}
