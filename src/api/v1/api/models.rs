use serde::Serialize;

#[derive(Serialize)]
pub struct HealthCheck {
    pub status: &'static str,
    pub message: &'static str,
}

#[derive(Serialize)]
pub struct Ping {
    pub version: String,
    pub server_date: String,
}
