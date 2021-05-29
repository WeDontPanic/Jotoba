#![allow(irrefutable_let_patterns)]

mod cli;
mod config;
mod webserver;

use import::has_required_data;

#[tokio::main]
pub async fn main() {
    let options = cli::parse();
    let database = models::connect();

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
        webserver::start(database).expect("webserver failed");
        return;
    }

    // User didn't read the docs
    println!("Nothing to do");
}
