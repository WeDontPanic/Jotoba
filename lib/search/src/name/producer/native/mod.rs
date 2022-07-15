pub mod split;

use crate::{
    engine::{names::native, search_task::cpushable::FilteredMaxCounter, SearchTask},
    executor::{out_builder::OutputBuilder, producer::Producer, searchable::Searchable},
    name::Search,
    query::{Query, QueryLang},
};

pub struct NativeProducer<'a> {
    query: &'a Query,
}

impl<'a> NativeProducer<'a> {
    pub fn new(query: &'a Query) -> Self {
        Self { query }
    }

    fn jp_task(&self) -> SearchTask<native::Engine> {
        SearchTask::<native::Engine>::new(&self.query.query_str)
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
        self.jp_task().find_to(out);
    }

    fn should_run(&self, _already_found: usize) -> bool {
        self.query.q_lang == QueryLang::Japanese
    }

    fn estimate_to(&self, out: &mut FilteredMaxCounter<<Self::Target as Searchable>::Item>) {
        self.jp_task().estimate_to(out);
    }
}
