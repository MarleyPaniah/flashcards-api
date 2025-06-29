use flashcards_api::{
    api::v1::{
        api::{routers::api_router, state::AppState},
        infra::database::get_postgresql_connection_pool,
    },
    config,
};
use std::{env, sync::Arc};
use tracing::info;
use tracing_subscriber;

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Load static config
    let config = Arc::new(config::load_config());

    // Create connection pool to the PostgreSQL database
    let pool = get_postgresql_connection_pool(&config.database_url);

    // Create an instance of the application state
    let state = AppState {
        config: config.clone(),
        pool,
    };

    // Build the app router
    let app = api_router().with_state(state);
    let server_address = format!("{}:{}", &config.host, &config.port);

    let listener = tokio::net::TcpListener::bind(&server_address)
        .await
        .unwrap();

    info!("Starting Flashcard API server.");
    info!("Listening on http://{}", &server_address);
    info!(
        "name={}, version={}",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION")
    );

    axum::serve(listener, app).await.unwrap();
}
