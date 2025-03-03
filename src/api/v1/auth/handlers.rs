use axum::debug_handler;
use axum::extract::State;
use axum::Json;
use tracing::info;

use crate::api::v1::api::state::AppState;

use crate::api::v1::api::errors::AppError;
use crate::api::v1::api::wrappers::AppJson;

use crate::api::v1::user::{
    models::NewUser,
    models::User,
    repository::{convert_new_user_to_new_user_db, UserRepository},
};

#[debug_handler]
pub async fn register_user(
    State(state): State<AppState>,
    Json(payload): Json<NewUser>,
) -> Result<AppJson<User>, AppError> {
    info!("Received request to create user '{}'", payload.username);

    let new_user_db = convert_new_user_to_new_user_db(payload);
    let result = UserRepository::insert_new_user(&state.pool, new_user_db).await;

    let user = result?;

    Ok(AppJson(user))
}
