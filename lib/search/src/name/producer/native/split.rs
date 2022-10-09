use engine::{
    pushable::{FilteredMaxCounter, Pushable},
    relevance::{data::SortData, item::RelItem, RelevanceEngine},
    task::SearchTask,
};
use ngindex2::{item::IndexItem, termset::TermSet};
use sentence_reader::{output::ParseResult, Parser};
use types::jotoba::names::Name;

use crate::{
    engine::names::native::Engine,
    executor::{out_builder::OutputBuilder, producer::Producer, searchable::Searchable},
    name::{order::japanese::NativeOrder, Search},
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
        C: Fn(&mut SearchTask<'static, Engine>, &mut P),
        P: Pushable<Item = O>,
    {
        let queries = self.queries();
        let query_count = queries.len();
        for (pos, query) in queries.into_iter().enumerate() {
            let mut task = SearchTask::<Engine>::new(&query)
                .with_limit(1)
                .with_custom_order(SplitOrder::new(query_count, pos));

            (cb)(&mut task, out);
        }
    }

    fn find_to<P>(&self, out: &mut P)
    where
        P: Pushable<Item = RelItem<&'static Name>>,
    {
        self.run(
            |engine, out| {
                engine.find_to(out);
            },
            out,
        );
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
        //already_found == 0
    }

    fn estimate_to(&self, out: &mut FilteredMaxCounter<<Self::Target as Searchable>::Item>) {
        self.run(|engine, out| engine.estimate_to(out), out);
    }
}

struct SplitOrder {
    q_count: usize,
    pos: usize,
}

impl SplitOrder {
    #[inline]
    fn new(q_count: usize, pos: usize) -> Self {
        Self { q_count, pos }
    }
}

impl RelevanceEngine for SplitOrder {
    type OutItem = &'static Name;
    type IndexItem = IndexItem<u32>;
    type Query = TermSet;

    #[inline]
    fn score<'item, 'query>(
        &self,
        item: &SortData<'item, 'query, Self::OutItem, Self::IndexItem, Self::Query>,
    ) -> f32 {
        let sim = NativeOrder.score(item);
        let rel = (self.q_count - self.pos) as f32;
        sim * rel * 0.001
    }
}
