use crate::api::v1::{
    api::state::AppState, services::short_id::error::ShortIdGeneratorServiceError,
};

use super::dto::ShortIdDto;

pub struct ShortIdGeneratorService;

impl ShortIdGeneratorService {
    pub async fn generate(state: &AppState) -> Result<ShortIdDto, ShortIdGeneratorServiceError> {
        let client = reqwest::Client::new();
        let request_url = format!(
            "http://{host}:{port}/generate",
            host = &state.config.sid_gen_host,
            port = &state.config.sid_gen_port,
        );
        let res = client
            .get(request_url)
            .send()
            .await
            .map_err(|e| ShortIdGeneratorServiceError::RequestError(e.to_string()))?
            .json::<ShortIdDto>()
            .await
            .map_err(|e| ShortIdGeneratorServiceError::ParseError(e.to_string()))?;

        Ok(res)
    }
}
