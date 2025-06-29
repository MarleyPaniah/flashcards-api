use crate::api::v1::{
    api::{errors::AppError, state::AppState},
    users::{
        models::{db::InsertUser, requests::NewUserRequestDto, responses::UserResponseDto},
        repository::UserRepository,
        utils::generate_password_hash,
    },
};

pub struct UserService {}

impl UserService {
    /// Add a user to the database.
    pub async fn create_user(
        state: &AppState,
        dto: NewUserRequestDto,
    ) -> Result<UserResponseDto, AppError> {
        Self::can_add_user(state, dto.username.clone(), dto.email.clone()).await?;

        let password_hash = generate_password_hash(&dto.password)
            .await
            .map_err(|err| AppError::UnknownError(err.to_string()))?;

        let insert_user = InsertUser {
            email: dto.email,
            username: dto.username,
            password_hash,
        };

        let user = UserRepository::insert_user(&state.pool, insert_user).await?;

        Ok(user.into())
    }

    /// Check that the username and email are free to use.
    pub async fn can_add_user(
        state: &AppState,
        username: String,
        email: String,
    ) -> Result<(), AppError> {
        if UserRepository::username_exists(&state.pool, username.clone()).await? {
            return Err(AppError::UserExists(format!(
                "Username '{}' already exists.",
                username
            )));
        }

        if UserRepository::email_in_use(&state.pool, email.clone()).await? {
            return Err(AppError::UserExists(format!(
                "Email '{}' already in use.",
                email
            )));
        }

        Ok(())
    }

    pub async fn get_user_by_username(
        state: &AppState,
        username: String,
    ) -> Result<UserResponseDto, AppError> {
        let user = UserRepository::get_user_by_username(&state.pool, username.clone()).await?;

        Ok(user.into())
    }
}
