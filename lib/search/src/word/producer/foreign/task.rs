use types::jotoba::languages::Language;

use crate::{
    engine::{words::foreign, SearchTask},
    query::Query,
    word::{filter, order},
};

/// Helper for creating SearchTask for foreign queries
pub struct ForeignSearch<'a> {
    query: &'a Query,
    query_str: &'a str,
    language: Language,
}

impl<'a> ForeignSearch<'a> {
    pub(crate) fn new(query: &'a Query, query_str: &'a str, language: Language) -> Self {
        Self {
            query,
            query_str,
            language,
        }
    }

    pub fn task(&self) -> SearchTask<foreign::Engine> {
        let mut task: SearchTask<foreign::Engine> =
            SearchTask::with_language(self.query_str, self.language).threshold(0.4f32);

        let lang = self.language;
        let orderer = order::foreign::ForeignOrder::new();
        task.with_custom_order(move |item| orderer.score(item, lang));

        let query_c = self.query.clone();
        task.set_result_filter(move |item| !filter::filter_word(item, &query_c));

        if !self.query.must_contain.is_empty() {
            let indexer = SearchTask::<foreign::Engine>::get_indexer(Some(self.language)).unwrap();

            let must_contain_ids: Vec<_> = self
                .query
                .must_contain
                .iter()
                .filter_map(|i| indexer.get_term(i))
                .collect();

            task.set_vector_filter(move |d_v, _q_vec| {
                // false -> remove
                must_contain_ids
                    .iter()
                    .any(|d| d_v.vector().has_dim(*d as u32))
            });
        }

        task
    }
}
