use super::filter::{self, FeQotTermsVecFilter};
use crate::{
    engine::sentences::foreign,
    executor::{out_builder::OutputBuilder, producer::Producer, searchable::Searchable},
    query::{Query, QueryLang},
    sentence::{order::foreign::ForeignOrder, Search},
};
use engine::{pushable::FilteredMaxCounter, task::SearchTask};
use types::jotoba::language::Language;

/// Producer for sentences by foreign keywords
pub struct ForeignProducer<'a> {
    query: &'a Query,
    language: Language,
}

impl<'a> ForeignProducer<'a> {
    pub fn new(query: &'a Query, language: Language) -> Self {
        Self { query, language }
    }

    fn task(&self) -> SearchTask<'static, foreign::Engine> {
        let query_str = &self.query.query_str;
        let query_c = self.query.clone();
        let vec_filter = FeQotTermsVecFilter::new(&self.query);
        let lang = self.query.lang();

        SearchTask::with_language(query_str, self.language)
            .with_result_filter(move |i| filter::filter_sentence(&query_c, *i))
            .with_item_filter(move |i| vec_filter.filter(i))
            .with_custom_order(ForeignOrder::new(lang))
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
        self.task().find_to(out);
    }

    fn should_run(&self, _already_found: usize) -> bool {
        self.query.form.is_normal() && self.query.q_lang == QueryLang::Foreign
    }

    fn estimate_to(&self, out: &mut FilteredMaxCounter<<Self::Target as Searchable>::Item>) {
        self.task().estimate_to(out);
    }
}
