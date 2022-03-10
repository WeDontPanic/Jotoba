pub mod kanji;

use error::api_error::RestError;
use search::{query::UserSettings, query_parser::QueryParser};
use serde::Serialize;
use types::{
    api::app::query::SearchPayload,
    jotoba::pagination::{page::Page, Pagination},
};

pub type Result<T> = std::result::Result<T, RestError>;

pub(crate) fn new_page<V: Serialize + Clone>(
    pl: &SearchPayload,
    v: V,
    items: u32,
    items_per_page: u32,
) -> Page<V> {
    let page = (pl.page.unwrap_or(1) as u32).max(1);
    Pagination::new_page(v, page, items, items_per_page, 100)
}

pub(crate) fn convert_payload(pl: &SearchPayload) -> QueryParser {
    let user_settings = convert_user_settings(&pl.settings);

    QueryParser::new(
        pl.query_str.clone(),
        types::jotoba::search::QueryType::Kanji,
        user_settings,
        pl.page.unwrap_or_default(),
        pl.word_index.unwrap_or_default(),
        false,
        pl.lang_overwrite,
    )
}

pub(crate) fn convert_user_settings(
    settings: &types::api::app::query::UserSettings,
) -> UserSettings {
    UserSettings {
        user_lang: settings.user_lang,
        show_english: settings.show_english,
        english_on_top: settings.english_on_top,
        page_size: settings.page_size,
        kanji_page_size: settings.kanji_page_size,
        show_example_sentences: settings.show_example_sentences,
        sentence_furigana: settings.sentence_furigana,
        ..Default::default()
    }
}
