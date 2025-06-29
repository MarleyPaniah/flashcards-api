use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ShortIdDto {
    pub id: String,
}
