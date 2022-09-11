pub mod sentence_reader;
pub mod task;

use crate::{
    engine::words::native::Engine,
    executor::{out_builder::OutputBuilder, producer::Producer, searchable::Searchable},
    query::{Query, QueryLang},
    word::Search,
};

use engine::{pushable::FilteredMaxCounter, task::SearchTask};
use task::NativeSearch;

/// Produces search results for native search input
pub struct NativeProducer<'a> {
    query: &'a Query,
}

impl<'a> NativeProducer<'a> {
    pub fn new(query: &'a Query) -> Self {
        Self { query }
    }

    fn task(&self) -> SearchTask<'static, Engine> {
        NativeSearch::new(self.query, &self.query.query_str).task()
    }
}

impl<'a> Producer for NativeProducer<'a> {
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
        self.task().estimate_to(out)
    }

    fn should_run(&self, already_found: usize) -> bool {
        if self.query.q_lang != QueryLang::Japanese || self.query.query_str.is_empty() {
            return false;
        }

        already_found < 5
    }
}
