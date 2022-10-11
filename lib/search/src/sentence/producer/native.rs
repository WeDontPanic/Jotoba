use super::filter;
use crate::{
    engine::sentences::native,
    executor::{out_builder::OutputBuilder, producer::Producer, searchable::Searchable},
    query::{Query, QueryLang},
    sentence::{order::native::NativeOrder, Search},
};
use engine::{pushable::FilteredMaxCounter, task::SearchTask};
use types::jotoba::languages::Language;

/// Producer for sentences by foreign keywords
pub struct NativeProducer<'a> {
    query: &'a Query,
    lang: Language,
}

impl<'a> NativeProducer<'a> {
    pub fn new(query: &'a Query, lang: Language) -> Self {
        Self { query, lang }
    }

    fn task(&self) -> SearchTask<'static, native::Engine> {
        let query = self.query.clone();
        let query_str = self.jp_reading();

        SearchTask::with_language(&query_str, self.lang)
            .with_result_filter(move |sentence| filter::filter_sentence(&query, *sentence))
            .with_custom_order(NativeOrder::new(self.query.lang()))
    }

    fn jp_reading(&self) -> String {
        let mut query_str = self.query.query_str.clone();

        if let Some(kanji_reading) = self.query.form.as_kanji_reading() {
            query_str = kanji_reading.literal.to_string();
        }

        query_str
    }
}

impl<'a> Producer for NativeProducer<'a> {
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
        self.query.form.is_normal() && self.query.q_lang == QueryLang::Japanese
    }

    fn estimate_to(&self, out: &mut FilteredMaxCounter<<Self::Target as Searchable>::Item>) {
        self.task().estimate_to(out);
    }
}
