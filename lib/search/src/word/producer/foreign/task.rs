use types::jotoba::languages::Language;

use crate::{
    engine::{words::foreign, SearchTask},
    query::Query,
    word::{filter::WordFilter, order},
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
            SearchTask::with_language(self.query_str, self.language);

        let lang = self.language;
        let orderer = order::foreign::ForeignOrder::new();
        task.with_custom_order(move |item| orderer.score(item, lang));

        let filter = WordFilter::new(self.query.clone());
        task.set_result_filter(move |item| !filter.filter_word(item));

        task
    }
}
