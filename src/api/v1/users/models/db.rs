use chrono::NaiveDateTime;
use diesel::prelude::{Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::schema::users;

#[derive(Serialize, Queryable, Selectable)]
#[diesel(table_name = users)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub password_hash: String,
    pub email: String,
    pub last_login_at: NaiveDateTime,
    pub is_verified: bool,
    pub is_deleted: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,
}

#[derive(Deserialize, Insertable)]
#[diesel(table_name = users)]
pub struct InsertUser {
    pub email: String,
    pub username: String,
    pub password_hash: String,
}
