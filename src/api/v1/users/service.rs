use crate::api::v1::{
    api::{
        errors::{
            basic::BasicError,
            infrastructure::InfrastructureError,
            users::UserError,
            utils::traits::{DynReturnableError, IntoBoxedError},
        },
        state::AppState,
    },
    users::{
        models::{db::InsertUser, requests::NewUserRequestDto, responses::UserResponseDto},
        repository::UserRepository,
        utils::generate_password_hash,
    },
};

pub struct UserService {}

impl UserService {
    pub async fn create_user(
        state: &AppState,
        dto: NewUserRequestDto,
    ) -> Result<UserResponseDto, DynReturnableError> {
        Self::can_add_user(state, dto.username.clone(), dto.email.clone()).await?;

        let password_hash = generate_password_hash(&state.config, &dto.password)
            .await
            .map_err(|err| BasicError::UnknownError(err.to_string()).boxed())?;

        let insert_user = InsertUser {
            email: dto.email,
            username: dto.username,
            password_hash,
        };

        let user = UserRepository::insert_user(&state.pool, insert_user)
            .await
            .map_err(Self::adapt_infra_error_any)?;

        Ok(user.into())
    }

    /**
      Check that the username and email are both free to use.
    */
    pub async fn can_add_user(
        state: &AppState,
        username: String,
        email: String,
    ) -> Result<(), DynReturnableError> {
        if UserRepository::username_exists(&state.pool, username.clone())
            .await
            .map_err(Self::adapt_infra_error_any)?
        {
            return Err(UserError::UserExists(username).boxed());
        }

        if UserRepository::email_in_use(&state.pool, email.clone())
            .await
            .map_err(Self::adapt_infra_error_any)?
        {
            return Err(UserError::EmailInUse(email).boxed());
        }

        Ok(())
    }

    pub async fn get_user_by_username(
        state: &AppState,
        username: String,
    ) -> Result<UserResponseDto, DynReturnableError> {
        let user = UserRepository::get_user_by_username(&state.pool, username.clone())
            .await
            .map_err(|err| Self::adapt_infra_error_user_info(err, username))?;

        Ok(user.into())
    }
}

// Error conversions
impl UserService {
    pub fn adapt_infra_error_any(_err: InfrastructureError) -> DynReturnableError {
        BasicError::UnknownError(
            "Error occurred when interacting with the duser datasource".to_string(),
        )
        .boxed()
    }

    /**
     When trying to get user info.
    */
    pub fn adapt_infra_error_user_info(
        err: InfrastructureError,
        username: String,
    ) -> DynReturnableError {
        match err {
            InfrastructureError::NotFound => UserError::UserDoesNotExist(username).boxed(),
            _ => BasicError::UnknownError("Error while fetching user by username".to_string())
                .boxed(),
        }
    }
}
