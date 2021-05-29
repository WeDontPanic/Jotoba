#[cfg(feature = "tokenizer")]
use std::path::Path;

#[cfg(feature = "tokenizer")]
use japanese::jp_parsing::{JA_NL_PARSER, NL_PARSER_PATH};

use crate::config::Config;
use actix_web::{
    dev::ServiceRequest,
    dev::{Service, ServiceResponse, Transform},
    http::header::{HeaderValue, CACHE_CONTROL},
    middleware, web as actixweb, App, Error, HttpServer,
};
use futures::{
    future::{ok, Ready},
    Future,
};
use models::DbPool;
use std::pin::Pin;
use std::task::{Context, Poll};

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
            .route("/index.html", actixweb::get().to(frontend::index::index))
            .route("/", actixweb::get().to(frontend::index::index))
            .route("/search", actixweb::get().to(frontend::search_ep::search))
            .route("/about", actixweb::get().to(frontend::about::about))
            .default_service(actix_web::Route::new().to(frontend::web_error::not_found))
            .route(
                "/api/kanji/by_radical",
                actixweb::post().to(api::radical::kanji_by_radicals),
            )
            .service(actixweb::scope("/assets").wrap(MyCacheInterceptor).service(
                actix_files::Files::new("", config_clone.server.get_html_files()),
            ))
    })
    .bind(&config.server.listen_address)?
    .run()
    .await
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

struct MyCacheInterceptor;

impl<S, B> Transform<S> for MyCacheInterceptor
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = MyCacheInterceptorMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(MyCacheInterceptorMiddleware { service })
    }
}

pub struct MyCacheInterceptorMiddleware<S> {
    service: S,
}

impl<S, B> Service for MyCacheInterceptorMiddleware<S>
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&mut self, req: ServiceRequest) -> Self::Future {
        let fut = self.service.call(req);

        Box::pin(async move {
            let mut res = fut.await?;
            let headers = res.headers_mut();
            // 1 Week should be enough
            headers.append(CACHE_CONTROL, HeaderValue::from_static("max-age=604800"));
            return Ok(res);
        })
    }
}
