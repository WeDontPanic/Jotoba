#[macro_use]
extern crate diesel;

use diesel::{r2d2::ConnectionManager, PgConnection};
use r2d2::{Pool, PooledConnection};

pub mod dict;
pub mod kanji;
pub mod name;
pub mod radical;
pub mod schema;
pub mod search_mode;
pub mod sense;
pub mod sentence;
pub mod sql;

pub type DbConnection = PooledConnection<ConnectionManager<PgConnection>>;
pub type DbPool = Pool<ConnectionManager<PgConnection>>;
use std::env;

use dotenv::dotenv;

/// Connect to the postgres database
pub fn connect() -> Pool<ConnectionManager<PgConnection>> {
    dotenv().ok();

    let manager = ConnectionManager::<PgConnection>::new(
        env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
    );

    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool")
}
