#![allow(irrefutable_let_patterns)]

mod cli;
mod webserver;

use import::has_required_data;

//#[tokio::main]
#[actix_web::main]
pub async fn main() {
    let options = cli::parse();
    /*
    let (pool, db_dsn) = models::connect().await;

    if !options.skip_migration {
        // Do DB migration
        models::migrate(&db_dsn).await;
    }

    // Run import process on --import/-i
    if options.import {
        import::import(&pool, &(&options).into()).await;
        return;
    }

    // Check for required data to be available
    if !has_required_data(&pool).await.expect("fatal DB error") {
        println!("Required data missing!");
        return;
    }

    */

    // Start the werbserver on --stat/-s
    if options.start {
        webserver::start().await.expect("webserver failed");
        return;
    }

    // User didn't read the docs
    println!("Nothing to do");
}
