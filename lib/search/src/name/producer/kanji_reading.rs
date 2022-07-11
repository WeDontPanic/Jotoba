use crate::{
    engine::{names::native, search_task::cpushable::FilteredMaxCounter, SearchTask},
    executor::{out_builder::OutputBuilder, producer::Producer, searchable::Searchable},
    name::Search,
    query::Query,
};
use japanese::furigana::generate::{assign_readings, ReadingRetrieve};
use resources::retrieve::kanji::KanjiRetrieve;
use types::jotoba::names::Name;

pub struct KreadingProducer<'a> {
    query: &'a Query,
}

impl<'a> KreadingProducer<'a> {
    pub fn new(query: &'a Query) -> Self {
        Self { query }
    }

    fn search_task(&self) -> Option<SearchTask<native::Engine>> {
        let k_reading = self.query.form.as_kanji_reading()?;

        let mut task = SearchTask::<native::Engine>::new(k_reading.literal.to_string());

        let literal = k_reading.literal;
        let reading = k_reading.reading.clone();

        task.set_result_filter(move |name| filter(name, &reading, literal).unwrap_or(false));

        Some(task)
    }
}

impl<'a> Producer for KreadingProducer<'a> {
    type Target = Search<'a>;

    fn produce(
        &self,
        out: &mut OutputBuilder<
            <Self::Target as Searchable>::Item,
            <Self::Target as Searchable>::ResAdd,
        >,
    ) {
        if let Some(task) = self.search_task() {
            task.find_to(out);
        }
    }

    fn should_run(&self, _already_found: usize) -> bool {
        self.query.form.is_kanji_reading()
    }

    fn estimate_to(&self, out: &mut FilteredMaxCounter<<Self::Target as Searchable>::Item>) {
        if let Some(task) = self.search_task() {
            task.estimate_to(out);
        }
    }
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
