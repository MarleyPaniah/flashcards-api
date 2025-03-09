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
    repository::{new_user_payload_to_new_user_db, UserField, UserRepository},
};

use super::utils::generate_password_hash;

#[debug_handler]
pub async fn register_user(
    State(state): State<AppState>,
    Json(payload): Json<NewUser>,
) -> Result<AppJson<User>, AppError> {
    info!("Received request to create user '{}'", payload.username);

    // Check if the username already exists
    debug!("Checking if username '{}' exists...", payload.username);
    if let Ok(_) =
        UserRepository::select_user(&state.pool, &payload.username, UserField::Username).await
    {
        return Err(AppError::UserExists(format!(
            "Username '{}' already exists.",
            payload.username
        )));
    }

    // Check if the email is already in use
    debug!("Checking if email '{}' exists...", payload.email);
    if let Ok(_) = UserRepository::select_user(&state.pool, &payload.email, UserField::Email).await
    {
        return Err(AppError::UserExists(format!(
            "Email '{}' already in use.",
            payload.email
        )));
    }

    // Generate password hash
    info!("Generating password hash...");
    let cost: u32 = 12;
    let max_threads = 4;
    let password_hash = generate_password_hash(&payload.password, Some(cost), Some(max_threads))
        .await
        .map_err(|err| AppError::UnknownError(err.to_string()))?;

    // Prepare the new user for insertion
    let new_user_db = new_user_payload_to_new_user_db(payload, password_hash);

    // Insert the new user into the database
    let result = UserRepository::insert_new_user(&state.pool, new_user_db).await;
    let user = result?;

    Ok(AppJson(user))
}
