use engine::task::SearchTask;
use types::jotoba::languages::Language;

use crate::{
    engine::words::foreign::Engine,
    query::Query,
    word::{filter::WordFilter, order::foreign::ForeignOrder},
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

    pub fn task(&self) -> SearchTask<'static, Engine> {
        let filter = WordFilter::new(self.query.clone());
        SearchTask::with_language(self.query_str, self.language)
            .with_custom_order(ForeignOrder)
            .with_result_filter(move |item| !filter.filter_word(*item))
    }
}
