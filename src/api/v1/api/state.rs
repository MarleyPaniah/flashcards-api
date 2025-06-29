use std::sync::Arc;

use deadpool_diesel::postgres::Pool;

use crate::config::Config;

// Struct to hold the application state
#[derive(Clone)]
pub struct AppState {
    pub config: Arc<Config>,
    pub pool: Pool,
}
