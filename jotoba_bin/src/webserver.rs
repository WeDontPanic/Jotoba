use actix_files::NamedFile;
use localization::TranslationDict;

use actix_web::{
    http::header::{ACCESS_CONTROL_ALLOW_ORIGIN, CACHE_CONTROL},
    middleware,
    web::{self as actixweb, Data},
    App, HttpRequest, HttpServer,
};
use config::Config;
use log::info;
use std::sync::Arc;

/// How long frontend assets are going to be cached by the clients. Currently 1 week
const ASSET_CACHE_MAX_AGE: u64 = 604800;

/// Start the webserver
pub(super) async fn start() -> std::io::Result<()> {
    setup_logger();

    let config = Config::new().await.expect("config failed");

    info!("Loading resources");

    resources::initialize_resources("./resources/storage_data").expect("Failed to load resources");

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
        use std::mem::ManuallyDrop;

        // We want to run sentry all the time so don't drop here
        let _guard = ManuallyDrop::new(sentry::init((
            sentry_config.dsn.as_str(),
            sentry::ClientOptions {
                release: sentry::release_name!(),
                ..Default::default()
            },
        )));

        std::env::set_var("RUST_BACKTRACE", "1");
    }

    search::word::load_indexes(&config).expect("Failed to load index files");

    if let Err(err) = api::completions::load_suggestions(&config) {
        log::error!("Failed loading suggestions: {}", err);
    }

    let config_clone = config.clone();
    HttpServer::new(move || {
        let app = App::new()
            // Data
            .app_data(Data::new(config_clone.clone()))
            // .app_data(Data::new(pool.clone()))
            .app_data(Data::new(locale_dict_arc.clone()))
            // Middlewares
            .wrap(middleware::Compress::default())
            .wrap(middleware::Logger::default())
            //.wrap(CookieSession::signed(&[0; 32]).secure(false))
            // Static files
            .route("/index.html", actixweb::get().to(frontend::index::index))
            .route("/docs.html", actixweb::get().to(docs))
            .route("/privacy", actixweb::get().to(privacy))
            .route("/", actixweb::get().to(frontend::index::index))
            .route(
                "/search/{query}",
                actixweb::get().to(frontend::search_ep::search),
            )
            .route("/about", actixweb::get().to(frontend::about::about))
            .default_service(actix_web::Route::new().to(frontend::web_error::not_found))
            // API
            .service(
                actixweb::scope("/api")
                    .wrap(
                        middleware::DefaultHeaders::new().header(ACCESS_CONTROL_ALLOW_ORIGIN, "*"),
                    )
                    .service(
                        actixweb::scope("search")
                            .route("words", actixweb::post().to(api::search::word::word_search))
                            .route(
                                "kanji",
                                actixweb::post().to(api::search::kanji::kanji_search),
                            )
                            .route("names", actixweb::post().to(api::search::name::name_search))
                            .route(
                                "sentences",
                                actixweb::post().to(api::search::sentence::sentence_search),
                            ),
                    )
                    .route(
                        "/kanji/by_radical",
                        actixweb::post().to(api::radical::kanji_by_radicals),
                    )
                    .route(
                        "/suggestion",
                        actixweb::post().to(api::completions::suggestion_ep),
                    ),
            )
            // Static files
            .service(
                actixweb::scope("/assets")
                    .wrap(
                        middleware::DefaultHeaders::new()
                            .header(CACHE_CONTROL, format!("max-age={}", ASSET_CACHE_MAX_AGE)),
                    )
                    .service(actix_files::Files::new(
                        "",
                        config_clone.server.get_html_files(),
                    )),
            );

        //#[cfg(feature = "sentry_error")]
        //let app = app.wrap(sentry_actix::Sentry::new());

        app
    })
    .bind(&config.server.listen_address)?
    .run()
    .await
}

async fn privacy(_req: HttpRequest) -> actix_web::Result<NamedFile> {
    Ok(NamedFile::open("html/privacypolicy.html")?)
}

async fn docs(_req: HttpRequest) -> actix_web::Result<NamedFile> {
    Ok(NamedFile::open("html/docs.html")?)
}

fn setup_logger() {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug"));
}

#[cfg(feature = "tokenizer")]
fn load_tokenizer() {
    use japanese::jp_parsing::{JA_NL_PARSER, NL_PARSER_PATH};
    use std::path::Path;

    if !Path::new(NL_PARSER_PATH).exists() {
        panic!("No NL dict was found! Place the following folder in he binaries root dir: ./unidic-mecab");
    }

    // Force parser to parse something to
    // prevent 1. search after launch taking up several seconds
    JA_NL_PARSER.parse("");
}
