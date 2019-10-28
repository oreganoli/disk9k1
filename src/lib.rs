extern crate diesel_migrations;

use diesel::prelude::*;
use dotenv;
use self::diesel_migrations::run_pending_migrations;

pub fn establish_connection() -> PgConnection {
    let url = dotenv::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let conn = PgConnection::establish(&url).expect(&format!("Error connecting to {}", url));
    run_pending_migrations(&conn).unwrap();
    conn.begin_test_transaction().unwrap();
    conn
}