use sentence_reader::{output::ParseResult, Parser};
use types::jotoba::names::Name;

use crate::{
    engine::{
        names::native,
        result_item::ResultItem,
        search_task::cpushable::{CPushable, FilteredMaxCounter},
        SearchTask,
    },
    executor::{out_builder::OutputBuilder, producer::Producer, searchable::Searchable},
    name::Search,
    query::Query,
};

pub struct SplitProducer<'a> {
    query: &'a Query,
}

impl<'a> SplitProducer<'a> {
    pub fn new(query: &'a Query) -> Self {
        Self { query }
    }

    fn queries(&self) -> Vec<String> {
        let splitted = Parser::new(&self.query.query_str);
        match splitted.parse() {
            ParseResult::Sentence(s) => s.iter().map(|p| p.get_normalized()).collect(),
            ParseResult::InflectedWord(w) => vec![w.get_normalized()],
            ParseResult::None => vec![],
        }
    }

    fn run<C, P, O>(&self, cb: C, out: &mut P)
    where
        C: Fn(&SearchTask<native::Engine>, &mut P),
        P: CPushable<Item = O>,
    {
        let queries = self.queries();
        let query_count = queries.len();
        for (pos, query) in queries.into_iter().enumerate() {
            let mut task = SearchTask::<native::Engine>::new(&query);
            task.with_custom_order(move |i| {
                let sim = i.vec_simiarity();
                let rel = query_count - pos;
                (rel as f32 * sim) as usize
            });

            (cb)(&task, out);
        }
    }

    fn find_to<P>(&self, out: &mut P)
    where
        P: CPushable<Item = ResultItem<&'static Name>>,
    {
        self.run(|engine, out| engine.find_to(out), out);
    }
}

impl<'a> Producer for SplitProducer<'a> {
    type Target = Search<'a>;

    fn produce(
        &self,
        out: &mut OutputBuilder<
            <Self::Target as Searchable>::Item,
            <Self::Target as Searchable>::ResAdd,
        >,
    ) {
        self.find_to(out)
    }

    fn should_run(&self, already_found: usize) -> bool {
        already_found < 10
    }

    fn estimate_to(&self, out: &mut FilteredMaxCounter<<Self::Target as Searchable>::Item>) {
        self.run(|engine, out| engine.estimate_to(out), out);
    }
}
