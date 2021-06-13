use std::str::FromStr;

use actix_web::{HttpMessage, HttpRequest};
use parse::jmdict::languages::Language;
use search::query::UserSettings;

/// Parses user settings from a `HttpRequest`
pub(super) fn parse(request: &HttpRequest) -> UserSettings {
    let show_english = request
        .cookie("show_english")
        .and_then(|i| i.value().parse().ok())
        .unwrap_or_else(|| UserSettings::default().show_english);

    let user_lang = request
        .cookie("default_lang")
        .and_then(|i| Language::from_str(i.value()).ok())
        .unwrap_or_default();

    let english_on_top = request
        .cookie("show_english_on_top")
        .and_then(|i| i.value().parse().ok())
        .unwrap_or_else(|| UserSettings::default().english_on_top)
        && show_english;

    UserSettings {
        user_lang,
        show_english,
        english_on_top,
        ..Default::default()
    }
}
