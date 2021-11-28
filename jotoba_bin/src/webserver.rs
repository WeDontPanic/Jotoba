use actix_files::NamedFile;
use localization::TranslationDict;

use actix_web::{
    http::header::{ACCESS_CONTROL_ALLOW_HEADERS, ACCESS_CONTROL_ALLOW_ORIGIN, CACHE_CONTROL},
    middleware::{self, Compat, Compress},
    web::{self as actixweb, Data},
    App, HttpRequest, HttpServer,
};
use config::Config;
use log::{debug, warn};
use std::{path::Path, sync::Arc, thread, time::Instant};

/// How long frontend assets are going to be cached by the clients. Currently 1 week
const ASSET_CACHE_MAX_AGE: u64 = 604800;

/// Start the webserver
pub(super) async fn start() -> std::io::Result<()> {
    setup_logger();

    let start = Instant::now();

    let mut threads = Vec::with_capacity(4);

    let config = Config::new(None).expect("config failed");

    let c2 = config.clone();
    threads.push(
        thread::Builder::new()
            .name("Resource loader".into())
            .spawn(move || load_resources(c2)),
    );

    let c2 = config.clone();
    threads.push(
        thread::Builder::new()
            .name("Suggestion loader".into())
            .spawn(move || load_suggestions(c2)),
    );

    let c2 = config.clone();
    threads.push(
        thread::Builder::new()
            .name("Index loader".into())
            .spawn(move || load_indexes(c2)),
    );

    threads.push(
        thread::Builder::new()
            .name("Tokenizer loader".into())
            .spawn(move || {
                load_tokenizer();
            }),
    );

    let locale_dict_arc = load_translations(&config);

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

    clean_img_scan_dir(&config);

    for thr in threads {
        thr.expect("Failed to spawn thread")
            .join()
            .expect("Failed to join threads");
    }

    let address = config.server.listen_address.clone();

    if let Err(err) = resources::news::News::load(config.server.get_news_folder()) {
        warn!("Failed to load news: {}", err);
    }

    debug!("Resource loading took {:?}", start.elapsed());

    HttpServer::new(move || {
        let app = App::new()
            // Data
            .app_data(Data::new(config.clone()))
            .app_data(Data::new(locale_dict_arc.clone()))
            // Middlewares
            .wrap(middleware::Logger::default())
            .service(
                actixweb::resource("/")
                    .wrap(Compat::new(middleware::Compress::default()))
                    .route(actixweb::get().to(frontend::index::index)),
            )
            .service(
                actixweb::resource("/docs.html")
                    .wrap(Compat::new(middleware::Compress::default()))
                    .route(actixweb::get().to(docs)),
            )
            .service(
                actixweb::resource("/privacy")
                    .wrap(Compat::new(middleware::Compress::default()))
                    .route(actixweb::get().to(privacy)),
            )
            .service(
                actixweb::resource("/service-worker.js")
                    .wrap(Compat::new(middleware::Compress::default()))
                    .route(actixweb::get().to(service_worker)),
            )
            .service(
                actixweb::resource("/search/{query}")
                    .wrap(Compat::new(middleware::Compress::default()))
                    .route(actixweb::get().to(frontend::search_ep::search_ep)),
            )
            .service(
                actixweb::resource("/search")
                    .wrap(Compat::new(middleware::Compress::default()))
                    .route(actixweb::get().to(frontend::search_ep::search_ep_no_js)),
            )
            .service(
                actixweb::resource("/about")
                    .wrap(Compat::new(middleware::Compress::default()))
                    .route(actixweb::get().to(frontend::about::about)),
            )
            .service(
                actixweb::resource("/news")
                    .wrap(Compat::new(middleware::Compress::default()))
                    .route(actixweb::get().to(frontend::news::news)),
            )
            .service(
                actixweb::resource("/help")
                    .wrap(Compat::new(middleware::Compress::default()))
                    .route(actixweb::get().to(frontend::help_page::help)),
            )
            .default_service(actix_web::Route::new().to(frontend::web_error::not_found))
            // API
            .service(
                actixweb::scope("/api")
                    .wrap(
                        middleware::DefaultHeaders::new()
                            .header(ACCESS_CONTROL_ALLOW_ORIGIN, "*")
                            .header(ACCESS_CONTROL_ALLOW_HEADERS, "Content-Type"),
                    )
                    .wrap(Compat::new(Compress::default()))
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
                        "/radical/search",
                        actixweb::post().to(api::radical::search::search_radical),
                    )
                    .route(
                        "/suggestion",
                        actixweb::post().to(api::completions::suggestion_ep),
                    )
                    .route("/img_scan", actixweb::post().to(api::img::scan_ep))
                    .route("/news/short", actixweb::post().to(api::news::short::news))
                    .route(
                        "/news/detailed",
                        actixweb::post().to(api::news::detailed::news),
                    ),
            )
            // Static files
            .service(
                actixweb::scope("/audio")
                    .wrap(
                        middleware::DefaultHeaders::new()
                            .header(CACHE_CONTROL, format!("max-age={}", ASSET_CACHE_MAX_AGE)),
                    )
                    .service(
                        actix_files::Files::new("", config.server.get_audio_files())
                            .show_files_listing(),
                    ),
            )
            .service(
                actixweb::scope("/assets")
                    .wrap(
                        middleware::DefaultHeaders::new()
                            .header(CACHE_CONTROL, format!("max-age={}", ASSET_CACHE_MAX_AGE)),
                    )
                    .wrap(Compat::new(Compress::default()))
                    .service(
                        actix_files::Files::new("", config.server.get_html_files())
                            .show_files_listing(),
                    ),
            )
            .service(
                actixweb::scope("/variable_assets/{oma}/assets")
                    .wrap(
                        middleware::DefaultHeaders::new()
                            .header(CACHE_CONTROL, format!("max-age={}", ASSET_CACHE_MAX_AGE)),
                    )
                    .wrap(Compat::new(Compress::default()))
                    .service(
                        actix_files::Files::new("", config.server.get_html_files())
                            .show_files_listing(),
                    ),
            );

        //#[cfg(feature = "sentry_error")]
        //let app = app.wrap(sentry_actix::Sentry::new());

        app
    })
    .bind(&address)?
    .run()
    .await
}

async fn service_worker(_req: HttpRequest) -> actix_web::Result<NamedFile> {
    Ok(NamedFile::open("html/assets/js/tools/service-worker.js")?)
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

pub fn load_tokenizer() {
    use japanese::jp_parsing::{JA_NL_PARSER, NL_PARSER_PATH};

    if !Path::new(NL_PARSER_PATH).exists() {
        panic!("No NL dict was found! Place the following folder in he binaries root dir: ./unidic-mecab");
    }

    // Force parser to parse something to
    // prevent 1. search after launch taking up several seconds
    JA_NL_PARSER.parse("");
}

/// Clears uploaded images which haven't been cleared yet
fn clean_img_scan_dir(config: &Config) {
    let path = config.get_img_scan_upload_path();
    let path = Path::new(&path);
    if !path.exists() || !path.is_dir() {
        return;
    }
    std::fs::remove_dir_all(&path).expect("Failed to clear img scan director");
}

pub fn load_resources(config: Config) {
    resources::initialize_resources(
        config.get_storage_data_path().as_str(),
        config.get_suggestion_sources(),
        config.get_radical_map_path().as_str(),
        config.get_sentences_path().as_str(),
    )
    .expect("Failed to load resources");
}

fn load_suggestions(config: Config) {
    if let Err(err) = api::completions::load_suggestions(&config) {
        warn!("Failed to load suggestions: {}", err);
    }
}

fn load_translations(config: &Config) -> Arc<TranslationDict> {
    let locale_dict = TranslationDict::new(
        config.server.get_locale_path(),
        localization::language::Language::English,
    )
    .expect("Failed to load localization files");

    Arc::new(locale_dict)
}

pub fn load_indexes(config: Config) {
    search::engine::load_indexes(&config).expect("Failed to load v2 index files");
}
