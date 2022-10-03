pub mod romaji;
pub mod task;

use crate::{
    executor::{out_builder::OutputBuilder, producer::Producer, searchable::Searchable},
    query::{Query, QueryLang},
    word::Search,
};
use engine::pushable::FilteredMaxCounter;
use task::ForeignSearch;
use types::jotoba::languages::Language;

/// Producer for words by foreign query
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
        //let mut p_mod = PushMod::new(out, |i: RelItem<WordOutput>| i.map_item(|i| i.word));

        let q_str = &self.query.query_str;
        let lang = self.query.get_search_lang();

        ForeignSearch::new(self.query, q_str, lang)
            .task()
            .find_to(out);

        // Add english results
        if lang != Language::English && self.query.show_english() {
            ForeignSearch::new(self.query, q_str, Language::English)
                .task()
                .find_to(out);
        }
    }

    fn estimate_to(&self, out: &mut FilteredMaxCounter<<Self::Target as Searchable>::Item>) {
        let q_str = &self.query.query_str;
        let lang = self.query.get_search_lang();

        ForeignSearch::new(self.query, q_str, lang)
            .task()
            .estimate_to(out);

        // Add english results
        if lang != Language::English && self.query.show_english() {
            ForeignSearch::new(self.query, q_str, Language::English)
                .task()
                .estimate_to(out);
        }
    }

    fn should_run(&self, _already_found: usize) -> bool {
        self.query.q_lang == QueryLang::Foreign && !self.query.query_str.is_empty()
    }
}
