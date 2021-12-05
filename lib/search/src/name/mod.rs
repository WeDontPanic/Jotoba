mod order;
pub mod result;

use crate::{
    engine::{
        guess::Guess,
        names::{foreign, native},
        SearchEngine, SearchTask,
    },
    query::QueryLang,
};

use self::result::NameResult;

use super::query::Query;
use error::Error;

use japanese::JapaneseExt;
use types::jotoba::names::Name;
use utils::to_option;

/// Search for names
#[inline]
pub fn search(query: &Query) -> Result<NameResult, Error> {
    if query.form.is_kanji_reading() {
        search_kanji(&query)
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

fn handle_search<T: SearchEngine<Output = Name> + Send>(
    task: SearchTask<T>,
) -> Result<NameResult, Error> {
    Ok(NameResult::from(task.find()?))
}

/// Search by kanji reading
fn search_kanji(query: &Query) -> Result<NameResult, Error> {
    let kanji_reading = query.form.as_kanji_reading().ok_or(Error::Unexpected)?;

    let query_str = kanji_reading.literal.to_string();
    let mut task = SearchTask::<native::Engine>::new(&query_str)
        .limit(query.settings.page_size as usize)
        .offset(query.page_offset);

    let literal = kanji_reading.literal;
    let reading = kanji_reading.reading.clone();
    task.set_result_filter(move |name| {
        if name.kanji.is_none() {
            return false;
        }
        let kanji = name.kanji.as_ref().unwrap();
        let kana = &name.kana;
        let readings = japanese::furigana::generate::retrieve_readings(
            &mut |i: String| {
                let retrieve = resources::get().kanji();
                let kanji = retrieve.by_literal(i.chars().next()?)?;
                if kanji.onyomi.is_none() && kanji.kunyomi.is_none() {
                    return None;
                }

                let kun = kanji
                    .clone()
                    .kunyomi
                    .unwrap_or_default()
                    .into_iter()
                    .chain(kanji.natori.clone().unwrap_or_default().into_iter())
                    .collect::<Vec<_>>();
                let kun = to_option(kun);

                Some((kun, kanji.onyomi.clone()))
            },
            kanji,
            kana,
        );

        if readings.is_none() {
            return false;
        }

        readings
            .unwrap()
            .iter()
            .any(|i| i.0.contains(&literal.to_string()) && i.1.contains(&reading))
    });

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
