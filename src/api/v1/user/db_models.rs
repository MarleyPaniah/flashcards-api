use super::schema::users;
use diesel::prelude::{Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Queryable, Selectable)]
#[diesel(table_name = users)]
pub struct UserDb {
    pub id: Uuid,
    pub username: String,
}

#[derive(Deserialize, Insertable)]
#[diesel(table_name = users)]
pub struct NewUserDb {
    pub email: String,
    pub username: String,
    pub password_hash: String,
}
