use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct NewUserRequestDto {
    pub email: String,
    pub username: String,
    pub password: String,
}
