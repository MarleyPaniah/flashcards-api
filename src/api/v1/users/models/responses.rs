use serde::Serialize;
use uuid::Uuid;

use super::db::User;

#[derive(Serialize)]
pub struct UserResponseDto {
    pub id: Uuid,
    pub username: String,
}

impl From<User> for UserResponseDto {
    fn from(user: User) -> Self {
        UserResponseDto {
            id: user.id,
            username: user.username,
        }
    }
}
