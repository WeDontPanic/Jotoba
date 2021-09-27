pub mod result;

use self::result::SentenceResult;

use super::query::Query;
use crate::{
    engine_v2::{sentences::foreign, sentences::native, SearchEngine, SearchTask},
    query::QueryLang,
};
use error::Error;
use resources::{models::sentences::Sentence, parse::jmdict::languages::Language};

/// Searches for sentences
pub async fn search(query: &Query) -> Result<SentenceResult, Error> {
    if query.language == QueryLang::Japanese {
        jp_search(query)
    } else {
        foreign_search(query)
    }
}

fn foreign_search(query: &Query) -> Result<SentenceResult, Error> {
    let mut search_task =
        SearchTask::<foreign::Engine>::with_language(&query.query, query.settings.user_lang)
            .limit(query.settings.items_per_page as usize)
            .offset(query.page_offset)
            .threshold(0.0);

    if query.settings.show_english && query.settings.user_lang != Language::English {
        search_task.add_language_query(&query.query, Language::English)
    }

    get_result(search_task, query)
}

fn jp_search(query: &Query) -> Result<SentenceResult, Error> {
    let search_task = SearchTask::<native::Engine>::new(&query.query)
        .limit(query.settings.items_per_page as usize)
        .offset(query.page_offset)
        .threshold(0.0);

    get_result(search_task, query)
}

fn get_result<T: SearchEngine<Output = Sentence>>(
    search: SearchTask<T>,
    query: &Query,
) -> Result<SentenceResult, Error> {
    let lang = query.settings.user_lang;
    let found = search.find()?;
    let len = found.len();
    let items = found
        .item_iter()
        .filter_map(|i| {
            let sentence =
                result::Sentence::from_m_sentence(i.clone(), lang, query.settings.show_english)?;
            Some(result::Item { sentence })
        })
        .collect::<Vec<_>>();

    Ok(SentenceResult { len, items })
}
