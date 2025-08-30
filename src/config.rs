use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct Config {
    pub host: String,
    pub port: usize,
    pub database_url: String,
    pub short_id_gen_host: String,
    pub short_id_gen_port: usize,
    pub password_hash_gen_cost: usize,
    pub password_hash_gen_max_threads: usize,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            host: "0.0.0.0".to_string(),
            port: 3000,
            database_url: "postgres://postgres:changeit@localhost:5432/flashcards_api".to_string(),
            short_id_gen_host: "localhost".to_string(),
            short_id_gen_port: 9889,
            password_hash_gen_cost: 12,
            password_hash_gen_max_threads: 4,
        }
    }
}

pub fn load_config() -> Config {
    dotenvy::dotenv().ok();
    envy::from_env::<Config>().expect("Invalid or missing env vars")
}
