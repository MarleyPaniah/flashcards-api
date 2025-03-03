use deadpool_diesel::postgres::Pool;

// Struct to hold the application state
#[derive(Clone)]
pub struct AppState {
    pub pool: Pool,
}
