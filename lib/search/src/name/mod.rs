mod kanji_reading;
pub mod result;

use super::query::Query;
use crate::{
    engine::{
        names::{foreign, native},
        SearchEngine, SearchTask,
    },
    query::{Form, QueryLang},
};
use japanese::JapaneseExt;
use result::NameResult;
use types::jotoba::{names::Name, search::guess::Guess};

/// Search for names
pub fn search(query: &Query) -> NameResult {
    match query.form {
        Form::Sequence(seq) => sequence_search(seq),
        Form::KanjiReading(ref reading) => kanji_reading::search(&query, reading),
        _ => {
            if query.q_lang == QueryLang::Japanese {
                handle_search(japanese_search(&query))
            } else {
                handle_search(foreign_search(&query))
            }
        }
    }
}

/// Search for sequence
fn sequence_search(seq: u32) -> NameResult {
    let retr = resources::get().names();

    let mut names = vec![];
    if let Some(name) = retr.by_sequence(seq) {
        names.push(name);
    }

    NameResult {
        items: names,
        total_count: 1,
    }
}

fn japanese_search(query: &Query) -> SearchTask<native::Engine> {
    SearchTask::<native::Engine>::new(&query.query_str)
        .threshold(0.05f32)
        .offset(query.page_offset)
        .limit(query.settings.page_size as usize)
}

fn foreign_search(query: &Query) -> SearchTask<foreign::Engine> {
    SearchTask::<foreign::Engine>::new(&query.query_str)
        .threshold(0.05f32)
        .offset(query.page_offset)
        .limit(query.settings.page_size as usize)
}

fn handle_search<T: SearchEngine<Output = &'static Name> + Send>(
    task: SearchTask<T>,
) -> NameResult {
    NameResult::from(task.find())
}

/// Guesses the amount of results a search would return with given `query`
pub fn guess_result(query: &Query) -> Option<Guess> {
    if query.form.is_kanji_reading() {
        return None;
    }

    if query.query_str.is_japanese() {
        japanese_search(query).estimate_result_count()
    } else {
        foreign_search(query).estimate_result_count()
    }
    .ok()
}
