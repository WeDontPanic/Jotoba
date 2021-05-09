use std::path::Path;

#[cfg(feature = "tokenizer")]
use crate::{JA_NL_PARSER, NL_PARSER_PATH};

use crate::{
    config::Config,
    models::{dict, kanji, sense},
    web, DbPool,
};
use actix_web::{middleware, web as actixweb, App, HttpServer};
use argparse::{ArgumentParser, Print, Store, StoreTrue};
use diesel::{r2d2::ConnectionManager, PgConnection};
use r2d2::{Pool, PooledConnection};

/// Start the webserver
#[actix_web::main]
pub(super) async fn start(db: DbPool) -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug"));

    let config = Config::new().await.expect("config failed");

    #[cfg(feature = "tokenizer")]
    load_tokenizer();

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
            .route(
                "/api/kanji/by_radical",
                actixweb::post().to(web::api::radical::kanji_by_radicals),
            )
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

    // Force parser to parse something to
    // prevent 1. search after launch taking up several seconds
    JA_NL_PARSER.parse("");
}
