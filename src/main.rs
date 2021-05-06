#![allow(irrefutable_let_patterns)]

#[macro_use]
extern crate diesel;

include!(concat!(env!("OUT_DIR"), "/templates.rs"));

pub mod cache;
mod config;
mod db;
pub mod error;
mod import;
pub mod japanese;
pub mod models;
mod parse;
pub mod schema;
pub mod search;
pub mod sql;
pub mod utils;
mod web;

use std::path::Path;

use actix_web::{middleware, web as actixweb, App, HttpServer};
use argparse::{ArgumentParser, Print, Store, StoreTrue};
use config::Config;
use diesel::{r2d2::ConnectionManager, PgConnection};
use models::{dict, kanji, sense};
use r2d2::{Pool, PooledConnection};

#[cfg(feature = "tokenizer")]
use once_cell::sync::Lazy;

pub type DbConnection = PooledConnection<ConnectionManager<PgConnection>>;
pub type DbPool = Pool<ConnectionManager<PgConnection>>;

#[cfg(feature = "tokenizer")]
const NL_PARSER_PATH: &str = "./unidic-mecab";

/// A global natural language parser
#[cfg(feature = "tokenizer")]
static JA_NL_PARSER: once_cell::sync::Lazy<igo_unidic::Parser> =
    Lazy::new(|| igo_unidic::Parser::new(NL_PARSER_PATH).unwrap());

#[derive(Default)]
struct Options {
    import: bool,
    jmdict_path: String,
    kanjidict_path: String,
    jlpt_paches_path: String,
    manga_sfx_path: String,
    jmnedict_path: String,
    sentences_path: String,
    accents_path: String,
    radicals_path: String,
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

    let sense_exists = sense::exists(&database).await.expect("fatal db err");
    let dict_exists = dict::exists(&database).await.expect("fatal db err");
    let kanji_exists = kanji::exists(&database).await.expect("fatal db err");

    // TODO make beautiful
    // import data
    if options.import {
        let mut imported_kanji = false;
        let mut imported_dicts = false;
        if !options.kanjidict_path.is_empty() {
            import::kanjidict::import(&database, options.kanjidict_path.clone()).await;
            imported_kanji = true;
        }

        if !options.jmdict_path.is_empty() {
            if !kanji_exists && !imported_kanji {
                println!("Kanji missing. Import the kanjidict first!");
                return;
            }

            import::jmdict::import(&database, options.jmdict_path.clone()).await;
            imported_dicts = true;
        }

        if !options.kanjidict_path.is_empty() && (dict_exists || !options.jmdict_path.is_empty()) {
            kanji::update_kun_links(&database)
                .await
                .expect("failed to update kun links");
        }

        if !options.radicals_path.is_empty() {
            if !kanji_exists && !imported_kanji {
                println!("Kanji entries are missing. Import them first!");
                return;
            }

            import::radicals::import(&database, options.radicals_path).await;
        }

        if !options.jlpt_paches_path.is_empty() {
            if (!kanji_exists && !imported_kanji) || (!dict_exists && !imported_dicts) {
                println!("Dict or kanji entries missing. Import them first!");
                return;
            }

            import::jlpt_patches::import(&database, options.jlpt_paches_path).await;
        }

        if !options.accents_path.is_empty() {
            if !dict_exists && !imported_dicts {
                println!("Dict entries missing. Import them first!");
                return;
            }

            import::accents::import(&database, options.accents_path).await;
        }

        if !options.sentences_path.is_empty() {
            if !dict_exists && !imported_dicts {
                println!("Dict entries missing. Import them first!");
                return;
            }

            import::sentences::import(&database, options.sentences_path).await;
        }

        if !options.manga_sfx_path.is_empty() {
            import::manga_sfx::import(&database, options.manga_sfx_path).await;
        }

        if !options.jmnedict_path.is_empty() {
            import::jmnedict::import(&database, options.jmnedict_path).await;
        }

        return;
    }

    // Check jmdict entries
    if !sense_exists || !dict_exists || !kanji_exists {
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
    let config = Config::new().await.expect("config failed");

    #[cfg(feature = "tokenizer")]
    load_tokenizer();

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug"));

    let config_clone = config.clone();
    HttpServer::new(move || {
        App::new()
            // Data
            .data(db.clone())
            .data(config_clone.clone())
            .app_data(db.clone())
            // Middlewares
            .wrap(middleware::Logger::default())
            .wrap(middleware::Compress::default())
            // Static files
            .route("/index.html", actixweb::get().to(web::index::index))
            .route("/", actixweb::get().to(web::index::index))
            .route("/search", actixweb::get().to(web::search::search))
            .service(actix_files::Files::new(
                "/assets",
                config_clone.server.get_html_files(),
            ))
    })
    .bind(&config.server.listen_address)?
    .run()
    .await
}

#[cfg(feature = "tokenizer")]
fn load_tokenizer() {
    println!("Loading Japanese natural language parser");
    if !Path::new(NL_PARSER_PATH).exists() {
        panic!("No NL dict was found! Place the following folder in he binaries root dir: ./unidic-mecab");
    }

    JA_NL_PARSER.parse("");
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
            "The path to import the kanjidict from. Required for --import",
        );

        ap.refer(&mut options.jmdict_path).add_option(
            &["--jmdict-path"],
            Store,
            "The path to import the jmdict from. Required for --import",
        );

        ap.refer(&mut options.jlpt_paches_path).add_option(
            &["--jlpt-patches-path"],
            Store,
            "The path to import the jlpt patches from. Required for --import",
        );

        ap.refer(&mut options.manga_sfx_path).add_option(
            &["--manga-sfx-path"],
            Store,
            "The path to import the manga sfx entries from. Required for --import",
        );

        ap.refer(&mut options.jmnedict_path).add_option(
            &["--jmnedict-path"],
            Store,
            "The path to import the name entries from. Required for --import",
        );

        ap.refer(&mut options.sentences_path).add_option(
            &["--sentences-path"],
            Store,
            "The path to import the sentences from. Required for --import",
        );

        ap.refer(&mut options.accents_path).add_option(
            &["--accents-path"],
            Store,
            "The path to import the accents from. Required for --import",
        );

        ap.refer(&mut options.radicals_path).add_option(
            &["--radicals-path"],
            Store,
            "The path to import the radicals from. Required for --import",
        );

        ap.parse_args_or_exit();
    }

    if options.import
        && options.jmdict_path.is_empty()
        && options.kanjidict_path.is_empty()
        && options.jlpt_paches_path.is_empty()
        && options.manga_sfx_path.is_empty()
        && options.jmnedict_path.is_empty()
        && options.sentences_path.is_empty()
        && options.accents_path.is_empty()
        && options.radicals_path.is_empty()
    {
        println!(
            "--manga-sfx-path, --jmdict-path, --jmnedict-path, --kanjidict-path, --accents-path, --sentences-path, --radicals-path or --jlpt-patches-path required"
        );
        return None;
    }

    Some(options)
}
