pub mod kanji;

use error::api_error::RestError;
use search::{query::UserSettings, query_parser::QueryParser};
use types::api::app::query::SearchPayload;

pub type Result<T> = std::result::Result<T, RestError>;

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
