pub mod kanji;
pub mod names;
pub mod sentences;
pub mod words;

use search::{
    query::UserSettings,
    query::{parser::QueryParser, Query},
};
use serde::Serialize;
use types::{
    api::app::search::{query::SearchPayload, responses::Response},
    jotoba::{
        pagination::{page::Page, Pagination},
        search::SearchTarget,
    },
};

const FIRST_PAGE: u32 = 1;
const LAST_PAGE: u32 = 100;

pub(crate) fn new_response<T: Serialize>(
    page: Page<T>,
    q_type: SearchTarget,
    query: &Query,
) -> Response<T> {
    Response::with_help_fn(page, |p| {
        if !p.is_empty() {
            return None;
        }
        search::build_help(q_type, &query)
    })
}

pub(crate) fn new_page<V: Serialize + Clone>(
    pl: &SearchPayload,
    v: V,
    items: u32,
    items_per_page: u32,
) -> Page<V> {
    let current_page = if items > 0 {
        (pl.page.unwrap_or(FIRST_PAGE)).max(FIRST_PAGE)
    } else {
        0
    };

    let mut pagination = Pagination::new_page(v, current_page, items, items_per_page, LAST_PAGE);

    if items == 0 {
        pagination.set_pages(0);
    }

    pagination
}

pub(crate) fn convert_payload(pl: &SearchPayload) -> QueryParser {
    let user_settings = convert_user_settings(&pl.settings);

    let mut q_parser = QueryParser::new(
        pl.query_str.clone(),
        types::jotoba::search::SearchTarget::Kanji,
        user_settings,
    )
    .with_page(pl.page.unwrap_or_default() as usize)
    .with_word_index(pl.word_index.unwrap_or_default());

    if let Some(lang) = pl.lang_overwrite {
        q_parser = q_parser.with_lang_overwrite(lang);
    }

    q_parser
}

pub(crate) fn convert_user_settings(
    settings: &types::api::app::search::query::UserSettings,
) -> UserSettings {
    UserSettings {
        user_lang: settings.user_lang,
        show_english: settings.show_english,
        english_on_top: true,
        page_size: settings.page_size,
        show_example_sentences: settings.show_example_sentences,
        sentence_furigana: settings.sentence_furigana,
        ..Default::default()
    }
}
