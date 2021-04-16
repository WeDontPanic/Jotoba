#![allow(irrefutable_let_patterns)]

#[macro_use]
extern crate diesel;

include!(concat!(env!("OUT_DIR"), "/templates.rs"));

mod db;
pub mod error;
mod import;
pub mod japanese;
pub mod models;
mod parse;
pub mod schema;
pub mod search;
mod web;

use actix_web::{middleware, web as actixweb, App, HttpServer};
use argparse::{ArgumentParser, Print, Store, StoreTrue};
use diesel::{r2d2::ConnectionManager, PgConnection};
use models::{dict, kanji, sense};
use r2d2::{Pool, PooledConnection};

pub type DbConnection = PooledConnection<ConnectionManager<PgConnection>>;
pub type DbPool = Pool<ConnectionManager<PgConnection>>;

#[derive(Default)]
struct Options {
    import: bool,
    jmdict_path: String,
    kanjidict_path: String,
    start: bool,
}

#[tokio::main]
pub async fn main() {
    let database = db::connect();

    // CLI options
    let options = match parse_args() {
        Some(options) => options,
        None => return,
    };

    // import data
    if options.import {
        if !options.jmdict_path.is_empty() {
            import::jmdict::import(&database, options.jmdict_path).await;
        }

        if !options.kanjidict_path.is_empty() {
            import::kanjidict::import(&database, options.kanjidict_path).await;
        }

        return;
    }

    // Check jmdict entries
    if !sense::exists(&database).await.expect("fatal db err")
        || !dict::exists(&database).await.expect("fatal db err")
        || !kanji::exists(&database).await.expect("fatal db err")
    {
        println!("jmdict or kanjidict entries missing. You need to import both!");
        return;
    }

    // Start
    if options.start {
        start_server(database).expect("webserver failed");
        return;
    }

    println!("Nothing to do");
}

/// Start the webserver
#[actix_web::main]
async fn start_server(db: DbPool) -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug"));
    HttpServer::new(move || {
        App::new()
            // Data
            .data(db.clone())
            .app_data(db.clone())
            // Middlewares
            .wrap(middleware::Logger::default())
            // Static files
            .route("/index.html", actixweb::get().to(web::index::index))
            .route("/", actixweb::get().to(web::index::index))
            .route("/search", actixweb::get().to(web::search::search))
            .service(actix_files::Files::new("/assets", "html/assets").show_files_listing())
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}

/// parse and verify cli args
fn parse_args() -> Option<Options> {
    let mut options = Options::default();
    {
        let mut ap = ArgumentParser::new();
        ap.set_description("A multilang japanese dictionary");
        ap.add_option(
            &["-V", "--version"],
            Print(env!("CARGO_PKG_VERSION").to_string()),
            "Show version",
        );

        ap.refer(&mut options.start)
            .add_option(&["--start", "-s"], StoreTrue, "Start the server");

        ap.refer(&mut options.import).add_option(
            &["--import", "-i"],
            StoreTrue,
            "Import some dictionary data",
        );

        ap.refer(&mut options.kanjidict_path).add_option(
            &["--kanjidict-path"],
            Store,
            "The pah to import the kanjidict from. Required for --import",
        );

        ap.refer(&mut options.jmdict_path).add_option(
            &["--jmdict-path"],
            Store,
            "The pah to import the jmdict from. Required for --import",
        );

        ap.parse_args_or_exit();
    }

    if options.import && options.jmdict_path.is_empty() && options.kanjidict_path.is_empty() {
        println!("--jmdict-path or --kanjidict-path required");
        return None;
    }

    Some(options)
}
