pub mod split;

use crate::{
    engine::names::native::Engine,
    executor::{out_builder::OutputBuilder, producer::Producer, searchable::Searchable},
    name::{order::japanese::NativeOrder, Search},
    query::{Query, QueryLang},
};
use engine::{pushable::FilteredMaxCounter, task::SearchTask};

pub struct NativeProducer<'a> {
    query: &'a Query,
}

impl<'a> NativeProducer<'a> {
    #[inline]
    pub fn new(query: &'a Query) -> Self {
        Self { query }
    }

    #[inline]
    fn jp_task(&self) -> SearchTask<'static, Engine> {
        SearchTask::<Engine>::new(&self.query.query_str)
            .with_custom_order(NativeOrder)
            .with_threshold(0.3)
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
