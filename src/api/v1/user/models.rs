use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize)]
pub struct User {
    pub id: Uuid,
    pub username: String,
}

#[derive(Deserialize, Serialize)]
pub struct NewUser {
    pub email: String,
    pub username: String,
    pub password: String,
}
