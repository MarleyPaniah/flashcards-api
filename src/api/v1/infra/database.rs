use deadpool_diesel::postgres::{Manager, Pool};
// use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

pub fn get_postgresql_connection_pool(database_url: &String) -> Pool {
    let manager = Manager::new(database_url, deadpool_diesel::Runtime::Tokio1);
    Pool::builder(manager).build().unwrap()
}

// fn run_migrations(
//     connection: &mut impl MigrationHarness<DB>,
//     migrations
// ) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
//     // This will run the necessary migrations.
//     connection.run_pending_migrations(MIGRATIONS)?;

//     Ok(())
// }
