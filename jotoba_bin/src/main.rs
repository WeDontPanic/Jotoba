#![allow(irrefutable_let_patterns)]

mod cli;
mod webserver;

use std::{env, str::FromStr};

use deadpool_postgres::{Manager, ManagerConfig, Pool, RecyclingMethod};
use import::has_required_data;
use tokio_postgres::NoTls;

//#[tokio::main]
#[actix_web::main]
pub async fn main() {
    let options = cli::parse();
    let database = models::connect();

    let connection_str = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = load_db(connection_str.clone());

    // Run import process on --import/-i
    if options.import {
        let database = database.get().unwrap();
        import::import(&database, &pool, &(&options).into()).await;
        return;
    }

    // Check for required data to be available
    if !has_required_data(&pool).await.expect("fatal DB error") {
        println!("Required data missing!");
        return;
    }

    // Start the werbserver on --stat/-s
    if options.start {
        webserver::start(database, pool)
            .await
            .expect("webserver failed");
        return;
    }

    // User didn't read the docs
    println!("Nothing to do");
}

fn load_db(connection_str: String) -> Pool {
    let pg_config =
        tokio_postgres::Config::from_str(&connection_str).expect("Failed to parse config");

    let mgr_config = ManagerConfig {
        recycling_method: RecyclingMethod::Fast,
    };
    let mgr = Manager::from_config(pg_config, NoTls, mgr_config);
    Pool::new(mgr, 16)
}
