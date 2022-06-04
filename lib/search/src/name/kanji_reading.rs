use error::Error;
use japanese::furigana::generate::retrieve_readings;
use types::jotoba::names::Name;
use utils::to_option;

use crate::{
    engine::{names::native, SearchTask},
    query::Query,
};

use super::result::NameResult;

/// Search by kanji reading
pub fn search(query: &Query) -> Result<NameResult, Error> {
    let kanji_reading = query.form.as_kanji_reading().ok_or(Error::Unexpected)?;

    let query_str = kanji_reading.literal.to_string();

    let mut task = SearchTask::<native::Engine>::new(&query_str)
        .limit(query.settings.page_size as usize)
        .offset(query.page_offset);

    let literal = kanji_reading.literal;
    let reading = kanji_reading.reading.clone();

    task.set_result_filter(move |name| filter(name, &reading, literal));

    Ok(NameResult::from(task.find()?))
}

fn filter(name: &Name, reading: &str, literal: char) -> bool {
    if name.kanji.is_none() {
        return false;
    }

    let kanji = name.kanji.as_ref().unwrap();
    let readings = match retrieve_readings(get_kanji_readings, kanji, &name.kana) {
        Some(r) => r,
        None => return false,
    };

    readings
        .iter()
        .any(|i| i.0.contains(&literal.to_string()) && i.1.contains(&reading))
}

fn get_kanji_readings(i: String) -> Option<(Option<Vec<String>>, Option<Vec<String>>)> {
    let retrieve = resources::get().kanji();
    let kanji = retrieve.by_literal(i.chars().next()?)?;

    if kanji.onyomi.is_empty() && kanji.kunyomi.is_empty() {
        return None;
    }

    let kun = kanji
        .clone()
        .kunyomi
        .into_iter()
        .chain(kanji.natori.clone().into_iter())
        .collect::<Vec<_>>();

    let kun = to_option(kun);
    let on = to_option(kanji.onyomi.clone());

    Some((kun, on))
}
