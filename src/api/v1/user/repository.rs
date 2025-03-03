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

// Converters
// To convert database representations to domain models and vice versa

// DB -> Model
fn convert_user_db_to_user(user_db: UserDb) -> User {
    User {
        id: user_db.id,
        username: user_db.username,
    }
}

// fn convert_new_user_db_to_new_user(new_user_db: NewUserDb) -> NewUser {
//     NewUser {
//         email: new_user_db.email,
//         username: new_user_db.username,
//         password: new_user_db.password_hash,
//     }
// }

// Model -> DB
pub fn convert_user_to_user_db(user: User) -> UserDb {
    UserDb {
        id: user.id,
        username: user.username,
    }
}

pub fn convert_new_user_to_new_user_db(new_user: NewUser) -> NewUserDb {
    NewUserDb {
        email: new_user.email,
        username: new_user.username,
        password_hash: new_user.password,
    }
}
