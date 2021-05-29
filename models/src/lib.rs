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
