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

mod embedded {
    use refinery::embed_migrations;
    embed_migrations!("../../db_migrations");
}

pub async fn connect() -> Pool {
    dotenv().ok();

    let connection_str = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // TODO remove
    //migrate(&connection_str).await;

    let pg_config =
        tokio_postgres::Config::from_str(&connection_str).expect("Failed to parse config");

    // TODO make more configurable
    let mgr_config = ManagerConfig {
        recycling_method: RecyclingMethod::Fast,
    };

    let mgr = Manager::from_config(pg_config, NoTls, mgr_config);

    Pool::new(mgr, 16)
}

pub async fn migrate(connection_str: &str) {
    let (mut client, connection) = tokio_postgres::connect(connection_str, NoTls)
        .await
        .expect("Coudln't connect to DB");

    // The connection object performs the actual communication with the database,
    // so spawn it off to run on its own.
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    let res = embedded::migrations::runner()
        .run_async(&mut client)
        .await
        .expect("Migration error");

    for migration in res.applied_migrations() {
        println!(
            "Migration applied: {}-{}",
            migration.name(),
            migration.version()
        );
    }
}
