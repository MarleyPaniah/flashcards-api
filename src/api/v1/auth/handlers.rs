use axum::debug_handler;
use axum::extract::State;
use axum::Json;
use tracing::{debug, info};

use crate::api::v1::api::state::AppState;

use crate::api::v1::api::errors::AppError;
use crate::api::v1::api::wrappers::AppJson;

use crate::api::v1::user::{
    models::NewUser,
    models::User,
    repository::{new_user_payload_to_new_user_db, UserRepository},
};

use super::utils::generate_password_hash;

#[debug_handler]
pub async fn register_user(
    State(state): State<AppState>,
    Json(payload): Json<NewUser>,
) -> Result<AppJson<User>, AppError> {
    info!("Received request to create user '{}'", payload.username);
    // debug!("Checking user exists...");
    // UserRepository::select_user_from_username(&state.pool, &payload.username)
    //     .await
    //     .expect_err("User '{payload.username}' already exists.");
    info!("Generating password hash...");
    let cost: u32 = 12;
    let max_threads = 4;
    let password_hash = generate_password_hash(&payload.password, Some(cost), Some(max_threads))
        .await
        .unwrap();

    let new_user_db = new_user_payload_to_new_user_db(payload, password_hash);

    let result = UserRepository::insert_new_user(&state.pool, new_user_db).await;

    let user = result?;

    Ok(AppJson(user))
}
