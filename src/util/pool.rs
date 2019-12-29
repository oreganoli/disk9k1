use postgres::{config::Config, NoTls};
use r2d2_postgres::PostgresConnectionManager;

use crate::prelude::*;

/// Creates a `Pool` pointing at our database.
pub fn create_pool() -> Pool {
    let url = std::env::var("DATABASE_URL").expect("DATABASE_URL is not set, panicking.");
    let manager = PostgresConnectionManager::new(url.parse().unwrap(), NoTls);
    Pool::builder()
        .max_size(12)
        .build(manager)
        .expect("Could not create a connection manager.")
}
