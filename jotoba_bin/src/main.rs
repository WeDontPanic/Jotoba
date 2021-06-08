#![allow(irrefutable_let_patterns)]

mod cli;
mod config;
mod webserver;

use std::env;

use import::has_required_data;
use tokio_postgres::NoTls;

#[tokio::main]
pub async fn main() {
    let options = cli::parse();
    let database = models::connect();

    let connection_str = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // Run import process on --import/-i
    if options.import {
        import::import(&database, &(&options).into()).await;
        return;
    }

    // Check for required data to be available
    if !has_required_data(&database).await.expect("fatal DB error") {
        println!("Required data missing!");
        return;
    }

    // Start the werbserver on --stat/-s
    if options.start {
        let (async_postgres, connection) = tokio_postgres::connect(&connection_str, NoTls)
            .await
            .expect("Couldn't connect to postgres");
        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("connection error: {}", e);
            }
        });
        webserver::start(database, async_postgres).expect("webserver failed");
        return;
    }

    // User didn't read the docs
    println!("Nothing to do");
}
