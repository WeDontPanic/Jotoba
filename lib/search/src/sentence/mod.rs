pub mod result;

use std::time::Instant;

use self::result::SentenceResult;

use super::query::Query;
use crate::{
    engine::{guess::Guess, sentences::foreign, sentences::native, SearchEngine, SearchTask},
    query::QueryLang,
};
use error::Error;
use resources::{models::sentences::Sentence, parse::jmdict::languages::Language};

/// Searches for sentences
pub fn search(query: &Query) -> Result<SentenceResult, Error> {
    let start = Instant::now();
    let res = if query.language == QueryLang::Japanese {
        get_result(jp_search(query), query)
    } else {
        get_result(foreign_search(query), query)
    }?;

    println!("Sentence search took: {:?}", start.elapsed());

    Ok(res)
}

fn foreign_search(query: &Query) -> SearchTask<foreign::Engine> {
    let mut search_task =
        SearchTask::<foreign::Engine>::with_language(&query.query, query.settings.user_lang)
            .limit(query.settings.items_per_page as usize)
            .offset(query.page_offset)
            .threshold(0.0);

    if query.settings.show_english && query.settings.user_lang != Language::English {
        search_task.add_language_query(&query.query, Language::English)
    }

    lang_filter(query, &mut search_task);

    search_task
}

fn jp_search(query: &Query) -> SearchTask<native::Engine> {
    let mut search_task = SearchTask::<native::Engine>::new(&query.query)
        .limit(query.settings.items_per_page as usize)
        .offset(query.page_offset)
        .threshold(0.0);

    lang_filter(query, &mut search_task);

    search_task
}

/// Sets a SearchTasks language filter
fn lang_filter<T: SearchEngine<Output = Sentence>>(query: &Query, search_task: &mut SearchTask<T>) {
    let lang = query.settings.user_lang;
    let show_english = query.settings.show_english;

    search_task.set_result_filter(move |sentence| {
        sentence.has_translation(lang)
            || (show_english && sentence.has_translation(Language::English))
    })
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

/// Guesses the amount of results a search would return with given `query`
pub fn guess_result(query: &Query) -> Option<Guess> {
    if query.language == QueryLang::Japanese {
        jp_search(query).estimate_result_count()
    } else {
        foreign_search(query).estimate_result_count()
    }
    .ok()
}
