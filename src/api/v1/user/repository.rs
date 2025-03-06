use crate::api::v1::infra::{error_utils::adapt_infra_error, errors::InfraError};
use diesel::prelude::*;

use super::{
    db_models::{NewUserDb, UserDb},
    models::{NewUser, User},
    schema::users,
};

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

    pub async fn select_user_from_username(
        pool: &deadpool_diesel::postgres::Pool,
        username: &str,
    ) -> Result<User, InfraError> {
        // Select an user from the user tables using its ID.

        let conn = pool.get().await.map_err(adapt_infra_error)?;
        let match_name = String::from(username);

        let res: UserDb = conn
            .interact(move |conn| {
                users::table
                    .filter(users::username.eq(match_name))
                    .select(UserDb::as_select())
                    .get_result(conn)
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
