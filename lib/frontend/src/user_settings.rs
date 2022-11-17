use std::str::FromStr;

use actix_web::HttpRequest;
use search::query::UserSettings;
use types::jotoba::languages::Language;

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

    let page_lang = request
        .cookie("page_lang")
        .and_then(|i| localization::language::Language::from_str(i.value()).ok())
        .unwrap_or_default();

    let english_on_top = request
        .cookie("show_english_on_top")
        .and_then(|i| i.value().parse().ok())
        .unwrap_or_else(|| UserSettings::default().english_on_top)
        && show_english;

    let items_per_page = request
        .cookie("items_per_page")
        .and_then(|i| i.value().parse().ok())
        .unwrap_or_else(|| UserSettings::default().page_size);

    let example_sentences_enabled = request
        .cookie("show_sentences")
        .and_then(|i| Some(i.value() == "true"))
        .unwrap_or_else(|| UserSettings::default().show_example_sentences);

    let sentence_furigana = request
        .cookie("sentence_furigana")
        .and_then(|i| Some(i.value() == "true"))
        .unwrap_or_else(|| UserSettings::default().sentence_furigana);

    UserSettings {
        user_lang,
        show_english,
        english_on_top,
        page_lang,
        page_size: items_per_page,
        show_example_sentences: example_sentences_enabled,
        sentence_furigana,
        ..Default::default()
    }
}
