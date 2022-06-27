pub mod kanji;
pub mod result;
mod tag_only;

use super::query::Query;
use crate::{
    engine::{sentences::foreign, sentences::native, SearchEngine, SearchTask},
    query::{tags::Tag, Form, QueryLang},
};
use error::Error;
use result::{Item, SentenceResult};
use types::jotoba::{languages::Language, search::guess::Guess, sentences::Sentence};

/// Searches for sentences
pub fn search(query: &Query) -> Result<SentenceResult, Error> {
    let res = match query.form {
        Form::TagOnly => tag_only::search(query)?,
        _ => normal_search(query),
    };
    Ok(res)
}

fn normal_search(query: &Query) -> SentenceResult {
    if query.language == QueryLang::Japanese {
        let query_str = jp_reading(query);
        get_result(jp_search(query, &query_str), query)
    } else {
        get_result(foreign_search(query), query)
    }
}

fn foreign_search(query: &Query) -> SearchTask<foreign::Engine> {
    let query_str = &query.query;

    let mut search_task =
        SearchTask::<foreign::Engine>::with_language(query_str, query.settings.user_lang)
            .limit(query.settings.page_size as usize)
            .offset(query.page_offset)
            .threshold(0.2);

    if query.settings.show_english && query.settings.user_lang != Language::English {
        search_task.add_language_query(&query.query, Language::English)
    }

    lang_filter(query, &mut search_task);
    sort_fn(query, query_str.to_string(), &mut search_task, false);

    search_task
}

fn jp_search<'a>(query: &Query, query_str: &'a str) -> SearchTask<'a, native::Engine> {
    let mut search_task = SearchTask::<native::Engine>::new(&query_str)
        .limit(query.settings.page_size as usize)
        .offset(query.page_offset)
        .threshold(0.2);

    lang_filter(query, &mut search_task);
    sort_fn(query, query_str.to_string(), &mut search_task, true);

    search_task
}

fn sort_fn<T: SearchEngine<Output = &'static Sentence> + Send>(
    query: &Query,
    query_str: String,
    search_task: &mut SearchTask<T>,
    japanese: bool,
) {
    let query = query.clone();
    search_task.with_custom_order(move |item| {
        let mut rel = (item.vec_simiarity() * 100000f32) as usize;

        let sentence = item.item();

        if sentence.has_translation(query.settings.user_lang) {
            rel += 550;
        }

        if japanese && sentence.japanese.contains(&query_str) {
            rel += 900;
        }

        rel
    })
}

/// Sets a SearchTasks language filter
fn lang_filter<T: SearchEngine<Output = &'static Sentence> + Send>(
    query: &Query,
    search_task: &mut SearchTask<T>,
) {
    let lang = query.settings.user_lang;
    let show_english = query.settings.show_english;

    let kanji_reading = query
        .form
        .as_kanji_reading()
        .and_then(|i| kanji::get_reading(i));

    search_task.set_result_filter(move |sentence| {
        let lang_filter = sentence.has_translation(lang)
            || (show_english && sentence.has_translation(Language::English));

        // No need to go further
        if !lang_filter {
            return false;
        }

        if let Some(reading) = &kanji_reading {
            return kanji::sentence_matches(sentence, &reading);
        }

        lang_filter
    })
}

fn get_result<T: SearchEngine<Output = &'static Sentence> + Send>(
    search: SearchTask<T>,
    query: &Query,
) -> SentenceResult {
    let lang = query.settings.user_lang;
    let found = search.find();
    let len = found.len();
    let show_english = query.settings.show_english();

    let items = found
        .into_iter()
        .filter_map(|i| map_sentence_to_item(i, lang, show_english))
        .collect::<Vec<_>>();
    let hidden = query.has_tag(Tag::Hidden);
    SentenceResult { len, items, hidden }
}

pub fn map_sentence_to_item(
    sentence: &Sentence,
    lang: Language,
    show_english: bool,
) -> Option<Item> {
    let sentence = result::Sentence::from_m_sentence(sentence.clone(), lang, show_english)?;
    Some(result::Item { sentence })
}

/// Guesses the amount of results a search would return with given `query`
pub fn guess_result(query: &Query) -> Option<Guess> {
    if query.language == QueryLang::Japanese {
        let query_str = jp_reading(query);
        jp_search(query, &query_str).estimate_result_count()
    } else {
        foreign_search(query).estimate_result_count()
    }
    .ok()
}

fn jp_reading(query: &Query) -> String {
    let mut query_str = query.query.clone();

    if let Some(kanji_reading) = query.form.as_kanji_reading() {
        query_str = kanji_reading.literal.to_string();
    }

    query_str
}
