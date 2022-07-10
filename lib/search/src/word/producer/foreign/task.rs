use types::jotoba::languages::Language;

use crate::{
    engine::{words::foreign, SearchTask},
    query::Query,
    word::order,
};

/// Helper for creating SearchTask for foreign queries
pub struct ForeignSearch<'a> {
    _query: &'a Query,
    query_str: &'a str,
    language: Language,
}

impl<'a> ForeignSearch<'a> {
    pub(crate) fn new(query: &'a Query, query_str: &'a str, language: Language) -> Self {
        Self {
            _query: query,
            query_str,
            language,
        }
    }

    pub fn task(&self) -> SearchTask<foreign::Engine> {
        let mut task: SearchTask<foreign::Engine> =
            SearchTask::with_language(self.query_str, self.language).threshold(0.4f32);

        // TODO: apply filter

        let lang = self.language;
        let orderer = order::foreign::ForeignOrder::new();
        task.with_custom_order(move |item| orderer.score(item, lang));

        task
    }
}
