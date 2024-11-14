use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use log::info;
use std::time::Duration;

pub fn establish_connection() -> Pool<ConnectionManager<SqliteConnection>> {
    info!("Connecting to database");
    let manager = ConnectionManager::<SqliteConnection>::new("library.db");
    let pool = Pool::builder()
        .max_size(20)
        .min_idle(Some(2))
        .connection_timeout(Duration::from_secs(5))
        .test_on_check_out(true)
        .build(manager)
        .expect("Failed to create pool.");

    info!("Database connection pool established successfully");
    pool
}
