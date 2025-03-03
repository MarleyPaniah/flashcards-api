use dotenvy::dotenv;
use flashcards_api::api::v1::{
    api::{routers::api_router, state::AppState},
    infra::database::get_postgresql_connection_pool,
};
use std::env;
use tracing::info;
use tracing_subscriber;

#[tokio::main]
async fn main() {
    // Load env vars at run time
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set.");
    let host = env::var("HOST").expect("HOST must be set.");
    let port = env::var("PORT").expect("PORT must be set.");

    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Create connection pool to the PostgreSQL database
    let pool = get_postgresql_connection_pool(&db_url);

    // Create an instance of the application state
    let state = AppState { pool };

    // Run migrations
    // TODO

    // Build the app router
    let app = api_router().with_state(state);
    let server_address = format!("{}:{}", host, port);

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
