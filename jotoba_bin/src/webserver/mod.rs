mod cache_control;

#[cfg(feature = "tokenizer")]
use std::path::Path;

#[cfg(feature = "tokenizer")]
use japanese::jp_parsing::{JA_NL_PARSER, NL_PARSER_PATH};
use localization::TranslationDict;

use crate::config::Config;
use actix_web::{middleware, web as actixweb, App, HttpServer};
use cache_control::CacheInterceptor;
use models::DbPool;
use std::{mem::ManuallyDrop, sync::Arc, time::Duration};

/// How long frontend assets are going to be cached by the clients. Currently 1 week
const ASSET_CACHE_MAX_AGE: u64 = 604800;

/// Start the webserver
#[actix_web::main]
pub(super) async fn start(db: DbPool) -> std::io::Result<()> {
    setup_logger();

    let config = Config::new().await.expect("config failed");

    #[cfg(feature = "tokenizer")]
    load_tokenizer();

    let locale_dict = TranslationDict::new(
        config.server.get_locale_path(),
        localization::language::Language::English,
    )
    .expect("Failed to load localization files");

    let locale_dict_arc = Arc::new(locale_dict);

    #[cfg(feature = "sentry_error")]
    if let Some(ref sentry_config) = config.sentry {
        let _guard = ManuallyDrop::new(sentry::init((
            sentry_config.dsn.as_str(),
            sentry::ClientOptions {
                release: sentry::release_name!(),
                ..Default::default()
            },
        )));
    }

    let config_clone = config.clone();
    HttpServer::new(move || {
        let app = App::new()
            // Data
            .data(db.clone())
            .data(config_clone.clone())
            .data(locale_dict_arc.clone())
            // Middlewares
            .wrap(middleware::Logger::default())
            .wrap(middleware::Compress::default())
            // Static files
            .route("/index.html", actixweb::get().to(frontend::index::index))
            .route("/", actixweb::get().to(frontend::index::index))
            .route("/search", actixweb::get().to(frontend::search_ep::search))
            .route("/about", actixweb::get().to(frontend::about::about))
            .default_service(actix_web::Route::new().to(frontend::web_error::not_found))
            // API
            .route(
                "/api/kanji/by_radical",
                actixweb::post().to(api::radical::kanji_by_radicals),
            )
            // Static files
            .service(
                actixweb::scope("/assets")
                    .wrap(CacheInterceptor(Duration::from_secs(ASSET_CACHE_MAX_AGE)))
                    .service(actix_files::Files::new(
                        "",
                        config_clone.server.get_html_files(),
                    )),
            );

        #[cfg(feature = "sentry_error")]
        let app = app.wrap(sentry_actix::Sentry::new());

        app
    })
    .bind(&config.server.listen_address)?
    .run()
    .await
}

fn setup_logger() {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug"));
}

#[cfg(feature = "tokenizer")]
fn load_tokenizer() {
    if !Path::new(NL_PARSER_PATH).exists() {
        panic!("No NL dict was found! Place the following folder in he binaries root dir: ./unidic-mecab");
    }

    // Force parser to parse something to
    // prevent 1. search after launch taking up several seconds
    JA_NL_PARSER.parse("");
}
