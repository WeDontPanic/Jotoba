use engine::task::SearchTask;
use types::jotoba::languages::Language;

use crate::{
    engine::words::foreign::foreign2::Engine,
    query::Query,
    word::{
        filter::WordFilter,
        order::{
            self,
            foreign::{ForeignOrder, ForeignOrder2},
        },
    },
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
        let task: SearchTask<Engine> = SearchTask::with_language(self.query_str, self.language)
            .with_custom_order(ForeignOrder2);

        //let lang = self.language;
        //task.with_custom_order(move |item| orderer.score(item, lang));

        // TODO
        //let filter = WordFilter::new(self.query.clone());
        //task.set_result_filter(move |item| !filter.filter_word(item));

        task
    }
}
