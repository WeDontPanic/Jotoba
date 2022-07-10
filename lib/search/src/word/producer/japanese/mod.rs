pub mod sentence_reader;
pub mod task;

use crate::{
    executor::{out_builder::OutputBuilder, producer::Producer, searchable::Searchable},
    query::{Query, QueryLang},
    word::Search,
};

use task::NativeSearch;

/// Produces search results for native search input
pub struct NativeProducer<'a> {
    query: &'a Query,
}

impl<'a> NativeProducer<'a> {
    pub fn new(query: &'a Query) -> Self {
        Self { query }
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
        NativeSearch::new(self.query, &self.query.query_str)
            .task()
            .find_to(out);
    }

    fn should_run(&self, already_found: usize) -> bool {
        already_found == 0 && self.query.q_lang == QueryLang::Japanese
    }
}
