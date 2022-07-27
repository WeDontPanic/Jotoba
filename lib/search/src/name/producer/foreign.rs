use crate::{
    engine::{names::foreign, search_task::cpushable::FilteredMaxCounter, SearchTask},
    executor::{out_builder::OutputBuilder, producer::Producer, searchable::Searchable},
    name::Search,
    query::{Query, QueryLang},
};

pub struct ForeignProducer<'a> {
    query: &'a Query,
}

impl<'a> ForeignProducer<'a> {
    pub fn new(query: &'a Query) -> Self {
        Self { query }
    }

    fn foreign_task(&self) -> SearchTask<foreign::Engine> {
        SearchTask::<foreign::Engine>::new(&self.query.query_str)
            .threshold((0.3 * 100000.0) as usize)
    }
}

impl<'a> Producer for ForeignProducer<'a> {
    type Target = Search<'a>;

    fn produce(
        &self,
        out: &mut OutputBuilder<
            <Self::Target as Searchable>::Item,
            <Self::Target as Searchable>::ResAdd,
        >,
    ) {
        self.foreign_task().find_to(out);
    }

    fn should_run(&self, _already_found: usize) -> bool {
        self.query.q_lang != QueryLang::Japanese
    }

    fn estimate_to(&self, out: &mut FilteredMaxCounter<<Self::Target as Searchable>::Item>) {
        self.foreign_task().estimate_to(out);
    }
}
