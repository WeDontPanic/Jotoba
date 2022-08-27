use japanese::guessing::could_be_romaji;

use crate::{
    engine::words::native::Engine,
    executor::{out_builder::OutputBuilder, producer::Producer, searchable::Searchable},
    query::{Query, QueryLang},
    word::{producer::japanese::task::NativeSearch, Search},
};
use engine::{pushable::FilteredMaxCounter, task::SearchTask};

pub struct RomajiProducer<'a> {
    query: &'a Query,
}

impl<'a> RomajiProducer<'a> {
    pub fn new(query: &'a Query) -> Self {
        Self { query }
    }

    fn hira_query(&self) -> String {
        japanese::to_hira_fmt(&self.query.query_str)
    }

    fn kk_query(&self) -> String {
        japanese::to_kk_fmt(&self.query.query_str)
    }

    fn kk_task(&self) -> SearchTask<'static, Engine> {
        let hira_query_str = self.kk_query();
        NativeSearch::new(self.query, &hira_query_str).task()
    }

    fn hira_task(&self) -> SearchTask<'static, Engine> {
        let hira_query_str = self.hira_query();
        NativeSearch::new(self.query, &hira_query_str)
            .with_custom_original_query(&hira_query_str)
            .task()
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
        self.hira_task().find_to(out);
        self.kk_task().find_to(out);
    }

    fn estimate_to(&self, out: &mut FilteredMaxCounter<<Self::Target as Searchable>::Item>) {
        self.hira_task().estimate_to(out);
        self.kk_task().estimate_to(out);
    }

    fn should_run(&self, already_found: usize) -> bool {
        already_found < 100
            // Don't run on jp input
            && self.query.q_lang == QueryLang::Foreign
            && could_be_romaji(&self.query.query_str)
    }
}
