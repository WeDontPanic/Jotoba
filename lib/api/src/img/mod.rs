#![allow(dead_code)]
pub mod request;

use std::path::Path;

use actix_multipart::Multipart;
use actix_web::web::{self, Json};
use config::Config;
use error::api_error::RestError;
use itertools::Itertools;
use once_cell::sync::Lazy;
use regex::Regex;
use types::api::image::{Request, Response};

// MAX 2MB
const MAX_UPLOAD_SIZE: usize = 2 * 1024 * 1024;

// Filter japanese from image text
const FILTER_JP_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new("[あ-ん一-龯一-龯０-９Ａ-ｚア-ン』『]").unwrap());

/// Get search suggestions endpoint
pub async fn scan_ep(
    payload: Multipart,
    args: web::Query<Request>,
    config: web::Data<Config>,
) -> Result<Json<Response>, actix_web::Error> {
    // Load payload
    let local_file = request::read_payload(&config, payload).await?;

    // Scan image
    let local_file_cloned = local_file.clone();
    let res = web::block(move || scan_image(local_file_cloned, &args, &config)).await;

    // Cleanup file
    web::block(move || std::fs::remove_file(local_file)).await??;

    // Handle result after cleaning up files
    Ok(Json(res??))
}

/// Scans an image and returns a `Response` with the recognized text or an error
#[cfg(feature = "img_scan")]
fn scan_image<P: AsRef<Path>>(
    file: P,
    req: &Request,
    config: &Config,
) -> Result<Response, RestError> {
    let tess_data = config.server.tess_data.as_ref().map(|i| i.as_str());
    let mut lt = leptess::LepTess::new(tess_data, "jpn").map_err(|_| RestError::Internal)?;
    lt.set_image(file).map_err(|_| RestError::NoTextFound)?;

    if lt.get_source_y_resolution() <= 0 {
        lt.set_source_resolution(70)
    }

    if lt.mean_text_conf() < req.threshold {
        return Err(RestError::NoTextFound);
    }

    let text = lt
        .get_utf8_text()
        .ok()
        .and_then(|text| format_text(text))
        .ok_or(RestError::NoTextFound)?;

    Ok(Response { text })
}

/// Format non-japanese characters from scanned result
fn format_text(text: String) -> Option<String> {
    let modded_text = FILTER_JP_REGEX
        .captures_iter(&text)
        .into_iter()
        .map(|i| {
            i.iter()
                .filter_map(|j| Some(j?.as_str().to_string()))
                .collect::<Vec<_>>()
        })
        .flatten()
        .join("");

    (!modded_text.is_empty()).then(|| modded_text)
}

/// Scans an image and returns a `Response` with the recognized text or an error
#[cfg(not(feature = "img_scan"))]
fn scan_image<P: AsRef<Path>>(
    _file: P,
    _req: &Request,
    _config: &Config,
) -> Result<Response, RestError> {
    Ok(Response {
        text: String::from("unsupported"),
    })
}
