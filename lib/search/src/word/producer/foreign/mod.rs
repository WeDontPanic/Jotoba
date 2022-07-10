pub mod romaji;
pub mod task;

use types::jotoba::languages::Language;

use crate::{
    engine::{
        result_item::ResultItem, search_task::pushable::PushMod, words::foreign::output::WordOutput,
    },
    executor::{out_builder::OutputBuilder, producer::Producer, searchable::Searchable},
    query::{Query, QueryLang},
    word::Search,
};

use task::ForeignSearch;

pub struct ForeignProducer<'a> {
    query: &'a Query,
}

impl<'a> ForeignProducer<'a> {
    pub fn new(query: &'a Query) -> Self {
        Self { query }
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
        let mut p_mod = PushMod::new(out, |i: ResultItem<WordOutput>| i.map_item(|i| i.word));

        let f_search = ForeignSearch::new(self.query, &self.query.query_str, self.query.lang());
        f_search.task().find_to(&mut p_mod);

        // Add english results
        if self.query.lang() != Language::English && self.query.show_english() {
            let f_search = ForeignSearch::new(self.query, &self.query.query_str, Language::English);
            f_search.task().find_to(&mut p_mod);
        }
    }

    fn should_run(&self, _already_found: usize) -> bool {
        self.query.q_lang == QueryLang::Foreign
    }
}
