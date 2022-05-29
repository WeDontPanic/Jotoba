#![allow(irrefutable_let_patterns)]

//#[global_allocator]
//static ALLOC: snmalloc_rs::SnMalloc = snmalloc_rs::SnMalloc;

mod check;
mod cli;
mod webserver;

#[actix_web::main]
pub async fn main() {
    let options = cli::parse();

    // Check resources on --check/-c
    if options.check_resources {
        check::check();
        return;
    }

    // Start the werbserver on --stat/-s
    if options.start {
        webserver::start(options).await.expect("webserver failed");
        return;
    }

    // User didn't read the docs
    println!("Nothing to do. Use `-s` to start the dictionary");
}
