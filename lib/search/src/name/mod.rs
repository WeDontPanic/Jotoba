mod kanji_reading;
pub mod result;

use super::query::Query;
use crate::{
    engine::{
        guess::Guess,
        names::{foreign, native},
        SearchEngine, SearchTask,
    },
    query::QueryLang,
};
use error::Error;
use japanese::JapaneseExt;
use result::NameResult;
use types::jotoba::names::Name;

/// Search for names
#[inline]
pub fn search(query: &Query) -> Result<NameResult, Error> {
    if query.form.is_kanji_reading() {
        kanji_reading::search(&query)
    } else {
        if query.language == QueryLang::Japanese {
            handle_search(japanese_search(&query))
        } else {
            handle_search(foreign_search(&query))
        }
    }
}

fn japanese_search(query: &Query) -> SearchTask<native::Engine> {
    SearchTask::<native::Engine>::new(&query.query)
        .threshold(0.05f32)
        .offset(query.page_offset)
        .limit(query.settings.page_size as usize)
}

fn foreign_search(query: &Query) -> SearchTask<foreign::Engine> {
    SearchTask::<foreign::Engine>::new(&query.query)
        .threshold(0.05f32)
        .offset(query.page_offset)
        .limit(query.settings.page_size as usize)
}

fn handle_search<T: SearchEngine<Output = &'static Name> + Send>(
    task: SearchTask<T>,
) -> Result<NameResult, Error> {
    Ok(NameResult::from(task.find()?))
}

/// Guesses the amount of results a search would return with given `query`
pub fn guess_result(query: &Query) -> Option<Guess> {
    if query.form.is_kanji_reading() {
        return None;
    }

    if query.query.is_japanese() {
        japanese_search(query).estimate_result_count()
    } else {
        foreign_search(query).estimate_result_count()
    }
    .ok()
}
