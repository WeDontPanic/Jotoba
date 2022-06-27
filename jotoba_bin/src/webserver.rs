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

use crate::{check, cli::Options};

/// How long frontend assets are going to be cached by the clients. Currently 1 week
const ASSET_CACHE_MAX_AGE: u64 = 604800;

/// Start the webserver
pub(super) async fn start(options: Options) -> std::io::Result<()> {
    if options.debug {
        println!("DEBUG MODE ENABLED");
        rayon::ThreadPoolBuilder::new()
            .num_threads(1)
            .build_global()
            .unwrap();
    }

    setup_logger();

    let start = Instant::now();

    let config = Config::new(None).expect("config failed");
    if options.debug {
        println!("{config:#?}");
    }

    prepare_data(&config);

    let locale_dict_arc = load_translations(&config);

    #[cfg(feature = "sentry_error")]
    setup_sentry(&config);

    let address = config.server.listen_address.clone();

    if !check() {
        return Ok(());
    }

    debug!("Resource loading took {:?}", start.elapsed());
    debug_info();

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
                actixweb::resource("/direct/{type}/{id}")
                    .wrap(Compat::new(middleware::Compress::default()))
                    .route(actixweb::get().to(frontend::direct::direct_ep)),
            )
            .service(
                actixweb::resource("/about")
                    .wrap(Compat::new(middleware::Compress::default()))
                    .route(actixweb::get().to(frontend::about::about)),
            )
            .service(
                actixweb::resource("/news")
                    .wrap(Compat::new(middleware::Compress::default()))
                    .route(actixweb::get().to(frontend::news_ep::news)),
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
                            .add((ACCESS_CONTROL_ALLOW_ORIGIN, "*"))
                            .add((ACCESS_CONTROL_ALLOW_HEADERS, "Content-Type")),
                    )
                    .wrap(Compat::new(Compress::default()))
                    .route("/", actixweb::get().to(docs))
                    .default_service(actix_web::Route::new().to(docs))
                    .service(
                        actixweb::scope("app")
                            .route(
                                "kanji",
                                actixweb::post().to(api::app::search::kanji::search),
                            )
                            .route(
                                "names",
                                actixweb::post().to(api::app::search::names::search),
                            )
                            .route(
                                "sentences",
                                actixweb::post().to(api::app::search::sentences::search),
                            )
                            .route(
                                "words",
                                actixweb::post().to(api::app::search::words::search),
                            )
                            .service(
                                actixweb::scope("details")
                                    .route(
                                        "word",
                                        actixweb::post().to(api::app::details::word::details),
                                    )
                                    .route(
                                        "sentence",
                                        actixweb::post()
                                            .to(api::app::details::sentences::details_ep),
                                    ),
                            ),
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
                    .service(
                        actixweb::scope("kanji")
                            .route(
                                "by_radical",
                                actixweb::post().to(api::radical::kanji_by_radicals),
                            )
                            .route(
                                "decompgraph",
                                actixweb::post().to(api::kanji::ids_tree::decomp_graph),
                            ),
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
                            .add((CACHE_CONTROL, format!("max-age={}", ASSET_CACHE_MAX_AGE))),
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
                            .add((CACHE_CONTROL, format!("max-age={}", ASSET_CACHE_MAX_AGE)))
                            .add((ACCESS_CONTROL_ALLOW_ORIGIN, "*"))
                            .add((ACCESS_CONTROL_ALLOW_HEADERS, "Content-Type")),
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
                            .add((CACHE_CONTROL, format!("max-age={}", ASSET_CACHE_MAX_AGE))),
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

pub(crate) fn prepare_data(ccf: &Config) {
    let cf = ccf.clone();
    thread::spawn(move || {
        indexes::storage::suggestions::load(cf.get_suggestion_sources())
            .expect("Failed to load suggestions");
        log::debug!("Suggestions loaded");
    });

    rayon::scope(move |s| {
        let cf = ccf.clone();
        s.spawn(move |_| {
            log::debug!("Loading Resources");
            load_resources(&cf.get_storage_data_path());
        });

        let cf = ccf.clone();
        s.spawn(move |_| {
            log::debug!("Loading Indexes");
            load_indexes(&cf);
        });

        let cf = ccf.clone();
        s.spawn(move |_| {
            log::debug!("Loading tokenizer");
            load_tokenizer(&cf);
        });

        let cf = ccf.clone();
        s.spawn(move |_| clean_img_scan_dir(&cf));

        let cf = ccf.clone();
        s.spawn(move |_| {
            log::debug!("Loading News");
            if let Err(err) = news::News::init(cf.server.get_news_folder()) {
                warn!("Failed to load news: {}", err);
            }
        });
    });
}

fn setup_logger() {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug"));
}

pub fn load_tokenizer(config: &Config) {
    sentence_reader::load_parser(&config.get_unidic_dict());
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

fn debug_info() {
    log::debug!("All features: {:?}", resources::Feature::all());
    log::debug!("Supported: {:?}", resources::get().get_features());
    log::debug!("Not supported: {:?}", resources::get().missing_features());
}

pub fn load_resources(src: &str) {
    let start = Instant::now();
    resources::load(src).expect("Failed to load resource storage");
    debug!("Resources took: {:?}", start.elapsed());
}

fn load_translations(config: &Config) -> Arc<TranslationDict> {
    let locale_dict = TranslationDict::new(
        config.server.get_locale_path(),
        localization::language::Language::English,
    )
    .expect("Failed to load localization files");

    Arc::new(locale_dict)
}

pub fn load_indexes(config: &Config) {
    indexes::storage::load(config.get_indexes_source()).expect("Failed to load index files");
}

fn check() -> bool {
    if !check::resources() {
        log::error!("Not all required data found! Exiting");
        return false;
    }

    if !indexes::get().check() {
        log::error!("Not all indexes are available!");
        return false;
    }

    if !indexes::get_suggestions().check() {
        log::error!("Not all suggestion indexes are available!");
        //return false;
    }

    true
}

#[cfg(feature = "sentry_error")]
fn setup_sentry(config: &Config) {
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
}
