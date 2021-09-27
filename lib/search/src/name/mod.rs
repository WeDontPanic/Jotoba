mod order;
pub mod result;

use crate::engine_v2::{
    names::{foreign, native},
    SearchEngine, SearchTask,
};

use self::result::NameResult;

use super::query::Query;
use error::Error;

use japanese::JapaneseExt;
use resources::models::names::Name;
use utils::to_option;

/// Search for names
#[inline]
pub async fn search(query: &Query) -> Result<NameResult, Error> {
    if query.form.is_kanji_reading() {
        search_kanji(&query).await
    } else {
        if query.query.is_japanese() {
            do_jp(query)
        } else {
            do_foreign(query)
        }
    }
}

fn do_jp(query: &Query) -> Result<NameResult, Error> {
    handle_search(
        SearchTask::<native::Engine>::new(&query.query)
            .threshold(0.05f32)
            .offset(query.page_offset)
            .limit(query.settings.items_per_page as usize),
    )
}

fn do_foreign(query: &Query) -> Result<NameResult, Error> {
    handle_search(
        SearchTask::<foreign::Engine>::new(&query.query)
            .threshold(0.05f32)
            .offset(query.page_offset)
            .limit(query.settings.items_per_page as usize),
    )
}

fn handle_search<T: SearchEngine<Output = Name>>(task: SearchTask<T>) -> Result<NameResult, Error> {
    Ok(NameResult::from(task.find()?))
}

/// Search by kanji reading
async fn search_kanji(query: &Query) -> Result<NameResult, Error> {
    let kanji_reading = query.form.as_kanji_reading().ok_or(Error::Unexpected)?;

    let query_str = kanji_reading.literal.to_string();
    let mut task = SearchTask::<native::Engine>::new(&query_str)
        .limit(query.settings.items_per_page as usize)
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
