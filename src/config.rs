use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub host: String,
    pub port: usize,

    pub database_url: String,

    pub sid_gen_host: String,
    pub sid_gen_port: usize,
}

pub fn load_config() -> Config {
    dotenvy::dotenv().ok();
    envy::from_env::<Config>().expect("Invalid or missing env vars")
}
