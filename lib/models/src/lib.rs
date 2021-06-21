pub mod dict;
pub mod kanji;
pub mod name;
pub mod queryable;
pub mod radical;
pub mod search_mode;
pub mod sense;
pub mod sentence;

use std::{env, str::FromStr};

use deadpool_postgres::{Manager, ManagerConfig, Pool, RecyclingMethod};
use dotenv::dotenv;
use tokio_postgres::NoTls;

pub fn connect() -> Pool {
    dotenv().ok();

    let connection_str = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pg_config =
        tokio_postgres::Config::from_str(&connection_str).expect("Failed to parse config");

    // TODO make more configurable
    let mgr_config = ManagerConfig {
        recycling_method: RecyclingMethod::Fast,
    };

    let mgr = Manager::from_config(pg_config, NoTls, mgr_config);
    Pool::new(mgr, 16)
}
