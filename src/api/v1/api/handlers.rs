use axum::{http::StatusCode, response::IntoResponse, Json};

use super::models::{HealthCheck, Ping};

pub async fn root_handler() -> impl IntoResponse {
    return (StatusCode::OK, "Ok.");
}

pub async fn health_checker_handler() -> impl IntoResponse {
    let res = HealthCheck {
        status: "success",
        message: "Server is running.",
    };
    (StatusCode::OK, Json(res))
}

pub async fn ping_handler() -> impl IntoResponse {
    let ping_res = Ping {
        version: env!("CARGO_PKG_VERSION").to_string(),
        server_date: chrono::offset::Local::now().to_string(),
    };

    (StatusCode::OK, Json(ping_res))
}

pub async fn error_404_handler() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "Not found.")
}
