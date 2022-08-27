use engine::{pushable::FilteredMaxCounter, task::SearchTask};

use crate::{
    engine::names::foreign::Engine,
    executor::{out_builder::OutputBuilder, producer::Producer, searchable::Searchable},
    name::{order::foreign::ForeignOrder, Search},
    query::{Query, QueryLang},
};

pub struct ForeignProducer<'a> {
    query: &'a Query,
}

impl<'a> ForeignProducer<'a> {
    pub fn new(query: &'a Query) -> Self {
        Self { query }
    }

    fn foreign_task(&self) -> SearchTask<'static, Engine> {
        let query = format_word(&self.query.query_str);
        SearchTask::<Engine>::new(&query)
            .with_custom_order(ForeignOrder)
            .with_threshold(0.5)
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

#[inline]
fn format_word(inp: &str) -> String {
    let mut out = String::from(inp.to_lowercase());
    for i in ".,[]() \t\"'\\/-;:".chars() {
        out = out.replace(i, " ");
    }
    out.to_lowercase()
}
