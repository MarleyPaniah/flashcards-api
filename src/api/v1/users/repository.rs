use deadpool_diesel::postgres::Pool;
use diesel::prelude::*;
use uuid::Uuid;

use crate::api::v1::api::errors::infrastructure::InfrastructureError;
use crate::api::v1::infra::database::database_interact;
use crate::schema::users;

use super::models::db::{InsertUser, User};

pub struct UserRepository;

impl UserRepository {
    /// Insert a user into the database.
    pub async fn insert_user(
        pool: &Pool,
        new_user: InsertUser,
    ) -> Result<User, InfrastructureError> {
        let user = database_interact(pool, |conn| {
            diesel::insert_into(users::table)
                .values(new_user)
                .returning(User::as_select())
                .get_result(conn)
        })
        .await?;

        Ok(user)
    }

    pub async fn get_user_by_id(pool: &Pool, user_id: Uuid) -> Result<User, InfrastructureError> {
        let user = database_interact(pool, move |conn| {
            users::table
                .filter(users::id.eq(user_id))
                .first::<User>(conn)
        })
        .await?;

        Ok(user)
    }

    pub async fn get_user_by_username(
        pool: &Pool,
        username: String,
    ) -> Result<User, InfrastructureError> {
        let user = database_interact(pool, move |conn| {
            users::table
                .filter(users::username.eq(username))
                .first::<User>(conn)
        })
        .await?;
        Ok(user)
    }

    pub async fn username_exists(
        pool: &Pool,
        username: String,
    ) -> Result<bool, InfrastructureError> {
        let count = database_interact(pool, move |conn| {
            users::table
                .filter(users::username.eq(username))
                .count()
                .get_result::<i64>(conn)
        })
        .await?;

        Ok(count > 0)
    }

    pub async fn email_in_use(pool: &Pool, email: String) -> Result<bool, InfrastructureError> {
        let count = database_interact(pool, move |conn| {
            users::table
                .filter(users::email.eq(email))
                .count()
                .get_result::<i64>(conn)
        })
        .await?;

        Ok(count > 0)
    }
}
