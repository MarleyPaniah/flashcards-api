use crate::api::v1::{
    api::errors::services::short_id::ShortIdGeneratorServiceError, api::state::AppState,
};

use super::dto::ShortIdDto;

pub struct ShortIdGeneratorService;

impl ShortIdGeneratorService {
    pub async fn generate(state: &AppState) -> Result<ShortIdDto, ShortIdGeneratorServiceError> {
        let client = reqwest::Client::new();
        let request_url = format!(
            "http://{host}:{port}/generate",
            host = &state.config.short_id_gen_host,
            port = &state.config.short_id_gen_port,
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
