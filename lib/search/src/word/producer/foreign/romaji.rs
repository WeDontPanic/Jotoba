use japanese::{guessing::could_be_romaji, JapaneseExt};

use crate::{
    engine::{search_task::cpushable::FilteredMaxCounter, words::native, SearchTask},
    executor::{out_builder::OutputBuilder, producer::Producer, searchable::Searchable},
    query::{Query, QueryLang},
    word::{producer::japanese::task::NativeSearch, Search},
};

pub struct RomajiProducer<'a> {
    query: &'a Query,
}

impl<'a> RomajiProducer<'a> {
    pub fn new(query: &'a Query) -> Self {
        Self { query }
    }

    fn romaji_query(&self) -> String {
        self.query.query_str.to_hiragana()
    }

    fn task(&self) -> SearchTask<native::Engine> {
        let hira_query_str = self.romaji_query();
        NativeSearch::new(self.query, &hira_query_str).task()
    }
}

impl<'a> Producer for RomajiProducer<'a> {
    type Target = Search<'a>;

    fn produce(
        &self,
        out: &mut OutputBuilder<
            <Self::Target as Searchable>::Item,
            <Self::Target as Searchable>::ResAdd,
        >,
    ) {
        self.task().find_to(out);
    }

    fn estimate_to(&self, out: &mut FilteredMaxCounter<<Self::Target as Searchable>::Item>) {
        self.task().estimate_to(out);
    }

    fn should_run(&self, already_found: usize) -> bool {
        already_found < 100
            // Don't run on jp input
            && self.query.q_lang == QueryLang::Foreign
            && could_be_romaji(&self.query.query_str)
    }
}
