use crate::{
    engine::{sentences::native, SearchTask},
    executor::{out_builder::OutputBuilder, producer::Producer, searchable::Searchable},
    query::{Query, QueryLang},
    sentence::Search,
};
use engine::pushable::FilteredMaxCounter;

/// Producer for sentences by foreign keywords
pub struct NativeProducer<'a> {
    query: &'a Query,
}

impl<'a> NativeProducer<'a> {
    pub fn new(query: &'a Query) -> Self {
        Self { query }
    }

    fn task(&self) -> SearchTask<native::Engine> {
        let query_str = self.jp_reading();

        let mut search_task = SearchTask::<native::Engine>::new(&query_str);

        let query_c = self.query.clone();
        search_task
            .set_result_filter(move |sentence| super::filter::filter_sentence(&query_c, sentence));

        let query_c = self.query.clone();
        search_task.with_custom_order(move |item| {
            let mut rel = item.vec_similarity();

            if !item.item().has_translation(query_c.lang()) {
                rel *= 0.99;
            }

            rel * 1_000_000.0
        });

        search_task
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
