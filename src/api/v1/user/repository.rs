use crate::api::v1::infra::{error_utils::adapt_infra_error, errors::InfraError};
use diesel::prelude::*;

use super::{
    db_models::{NewUserDb, UserDb},
    models::{NewUser, User},
    schema::users,
    schema::users::dsl as users_dsl,
};

// Enum used to filter the SELECT with in the DB.
pub enum UserField {
    Username,
    Email,
}

pub struct UserRepository;

impl UserRepository {
    pub async fn insert_new_user(
        pool: &deadpool_diesel::postgres::Pool,
        new_user: NewUserDb,
    ) -> Result<User, InfraError> {
        // Insert a new user into the database.

        // Get a database connection from the pool and handle any potential errors
        let conn = pool.get().await.map_err(adapt_infra_error)?;

        // Insert the new user and get the result of the operation
        let res = conn
            .interact(|conn| {
                diesel::insert_into(users::table)
                    .values(new_user)
                    .returning(UserDb::as_select())
                    .get_result(conn)
            })
            .await
            .map_err(adapt_infra_error)?
            .map_err(adapt_infra_error)?;

        Ok(convert_user_db_to_user(res))
    }

    pub async fn select_user(
        pool: &deadpool_diesel::postgres::Pool,
        value: &str,
        field: UserField,
    ) -> Result<User, InfraError> {
        let conn = pool.get().await.map_err(adapt_infra_error)?;
        let match_value = String::from(value);

        // DevNote: Tried to use box statement, but it felt too complex for this small task.
        let res: UserDb = conn
            .interact(move |conn| {
                use users_dsl::*;
                let mut query = users.into_boxed();

                query = match field {
                    UserField::Username => query.filter(username.eq(match_value)),
                    UserField::Email => query.filter(email.eq(match_value)),
                };

                query.select(UserDb::as_select()).get_result(conn)
            })
            .await
            .map_err(adapt_infra_error)?
            .map_err(adapt_infra_error)?;

        Ok(convert_user_db_to_user(res))
    }
}

pub fn new_user_payload_to_new_user_db(payload: NewUser, password_hash: String) -> NewUserDb {
    NewUserDb {
        email: payload.email,
        username: payload.username,
        password_hash,
    }
}

// DB -> Model
fn convert_user_db_to_user(user_db: UserDb) -> User {
    User {
        id: user_db.id,
        username: user_db.username,
    }
}
