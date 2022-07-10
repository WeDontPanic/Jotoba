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
        // convert WordOutput -> Word
        let mut p_mod = PushMod::new(out, |i: ResultItem<WordOutput>| i.map_item(|i| i.word));

        let q_str = &self.query.query_str;
        let lang = self.query.get_search_lang();

        ForeignSearch::new(self.query, q_str, lang)
            .task()
            .find_to(&mut p_mod);

        // Add english results
        if lang != Language::English && self.query.show_english() {
            ForeignSearch::new(self.query, q_str, Language::English)
                .task()
                .find_to(&mut p_mod);
        }
    }

    fn should_run(&self, _already_found: usize) -> bool {
        self.query.q_lang == QueryLang::Foreign
    }
}
