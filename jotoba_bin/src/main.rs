#![allow(irrefutable_let_patterns)]

mod cli;
mod webserver;

use std::env;

use import::has_required_data;

//#[tokio::main]
#[actix_web::main]
pub async fn main() {
    let options = cli::parse();
    let database = models::connect();

    let connection_str = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // Run import process on --import/-i
    if options.import {
        let database = database.get().unwrap();
        import::import(&database, &(&options).into()).await;
        return;
    }

    // Check for required data to be available
    if !has_required_data(&database.get().unwrap())
        .await
        .expect("fatal DB error")
    {
        println!("Required data missing!");
        return;
    }

    // Start the werbserver on --stat/-s
    if options.start {
        webserver::start(database, connection_str)
            .await
            .expect("webserver failed");
        return;
    }

    // User didn't read the docs
    println!("Nothing to do");
}
