use deadpool_diesel::postgres::{Manager, Pool};
use diesel::{result::Error as DieselError, PgConnection};

use crate::api::v1::api::errors::infrastructure::InfrastructureError;

use super::error_utils::adapt_infra_error;

pub fn get_postgresql_connection_pool(database_url: &String) -> Pool {
    let manager = Manager::new(database_url, deadpool_diesel::Runtime::Tokio1);
    Pool::builder(manager).build().unwrap()
}

/// Interact with the database.
///
/// Wrapper and helper function which reduces boilerplate
/// to interact with the deadpool_diesel pool.
pub async fn database_interact<T, F>(pool: &Pool, operation: F) -> Result<T, InfrastructureError>
where
    F: FnOnce(&mut PgConnection) -> Result<T, DieselError> + Send + 'static,
    T: Send + 'static,
{
    let conn = pool.get().await.map_err(adapt_infra_error)?;
    conn.interact(operation)
        .await
        .map_err(adapt_infra_error)?
        .map_err(adapt_infra_error)
}

#[macro_export]
macro_rules! database_async {
    ($pool:expr, $fn:path, $( $arg:expr ),*) => {
        crate::infra::database::database_interact($pool, move |conn| {
            $fn(conn, $( $arg ),*)
        }).await
    };
}
