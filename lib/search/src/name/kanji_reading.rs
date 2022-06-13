use super::result::NameResult;
use crate::{
    engine::{names::native, SearchTask},
    query::Query,
};
use error::Error;
use japanese::furigana::generate::{assign_readings, ReadingRetrieve};
use resources::retrieve::kanji::KanjiRetrieve;
use types::jotoba::names::Name;

/// Search by kanji reading
pub fn search(query: &Query) -> Result<NameResult, Error> {
    let kanji_reading = query.form.as_kanji_reading().ok_or(Error::Unexpected)?;

    let query_str = kanji_reading.literal.to_string();

    let mut task = SearchTask::<native::Engine>::new(&query_str)
        .limit(query.settings.page_size as usize)
        .offset(query.page_offset);

    let literal = kanji_reading.literal;
    let reading = kanji_reading.reading.clone();

    task.set_result_filter(move |name| filter(name, &reading, literal).unwrap_or(false));

    Ok(NameResult::from(task.find()))
}

/// Search result filter function
fn filter(name: &Name, reading: &str, literal: char) -> Option<bool> {
    let kanji = name.kanji.as_ref()?;

    let retrieve = NanoriRetrieve::new(resources::get().kanji());
    let readings = assign_readings(retrieve, kanji, &name.kana)?;

    Some(
        readings
            .iter()
            .any(|i| i.0.contains(&literal.to_string()) && i.1.contains(&reading)),
    )
}

/// Custom `ReadingRetrieve` implementing struct to include
// nanori readings in reading retrieve function
struct NanoriRetrieve<'a> {
    kanji_retrieve: KanjiRetrieve<'a>,
}

impl<'a> NanoriRetrieve<'a> {
    fn new(kanji_retrieve: KanjiRetrieve<'a>) -> Self {
        Self { kanji_retrieve }
    }
}

impl<'a> ReadingRetrieve for NanoriRetrieve<'a> {
    #[inline]
    fn onyomi(&self, lit: char) -> Vec<String> {
        self.kanji_retrieve.onyomi(lit)
    }

    #[inline]
    fn kunyomi(&self, lit: char) -> Vec<String> {
        self.kanji_retrieve.kunyomi(lit)
    }

    fn all(&self, lit: char) -> Vec<String> {
        let res = resources::get().kanji();
        let k = match res.by_literal(lit) {
            Some(k) => k,
            None => return vec![],
        };

        k.kunyomi
            .clone()
            .into_iter()
            .chain(k.onyomi.clone())
            .chain(k.nanori.clone())
            .collect()
    }
}
