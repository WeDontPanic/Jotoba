use crate::{
    engine::{sentences::foreign, sentences::native, SearchEngine, SearchTask},
    executor::{out_builder::OutputBuilder, producer::Producer, searchable::Searchable},
    query::Query,
    sentence::Search,
};
use types::jotoba::{languages::Language, search::guess::Guess, sentences::Sentence};

use super::kanji;

/// Producer for japanese or foreign sentences by keywords
pub struct SentenceProducer<'a> {
    query: &'a Query,
    japanese: bool,
}

impl<'a> Producer for SentenceProducer<'a> {
    type Target = Search<'a>;

    fn produce(
        &self,
        out: &mut OutputBuilder<
            <Self::Target as Searchable>::Item,
            <Self::Target as Searchable>::OutputAdd,
        >,
    ) {
        if self.japanese {
            let query_str = self.jp_reading();
            self.japanese(&query_str).find_to(out);
        } else {
            self.foreign().find_to(out);
        }
    }

    fn estimate(&self) -> Option<Guess> {
        if self.japanese {
            let query_str = self.jp_reading();
            self.japanese(&query_str).estimate_result_count().ok()
        } else {
            self.foreign().estimate_result_count().ok()
        }
    }

    fn should_run(&self, _already_found: usize) -> bool {
        self.query.form.is_normal()
    }
}

impl<'a> SentenceProducer<'a> {
    pub(crate) fn new(query: &'a Query, japanese: bool) -> Self {
        Self { query, japanese }
    }

    fn foreign(&self) -> SearchTask<foreign::Engine> {
        let query_str = &self.query.query_str;

        let mut search_task =
            SearchTask::<foreign::Engine>::with_language(query_str, self.query.settings.user_lang)
                .threshold(0.2);

        if self.query.settings.show_english && self.query.settings.user_lang != Language::English {
            search_task.add_language_query(&self.query.query_str, Language::English)
        }

        self.lang_filter(&mut search_task);
        self.sort_fn(query_str.to_string(), &mut search_task, false);

        search_task
    }

    fn japanese(&self, query_str: &str) -> SearchTask<native::Engine> {
        let mut search_task = SearchTask::<native::Engine>::new(&query_str).threshold(0.2);
        self.lang_filter(&mut search_task);
        self.sort_fn(query_str.to_string(), &mut search_task, true);
        search_task
    }

    fn sort_fn<T: SearchEngine<Output = &'static Sentence> + Send>(
        &self,
        query_str: String,
        search_task: &mut SearchTask<T>,
        japanese: bool,
    ) {
        let query = self.query.clone();
        search_task.with_custom_order(move |item| {
            let mut rel = (item.vec_simiarity() * 100000f32) as usize;

            let sentence = item.item();

            if sentence.has_translation(query.settings.user_lang) {
                rel += 550;
            }

            if japanese && sentence.japanese.contains(&query_str) {
                rel += 900;
            }

            rel
        })
    }

    /// Sets a SearchTasks language filter
    fn lang_filter<T: SearchEngine<Output = &'static Sentence> + Send>(
        &self,
        search_task: &mut SearchTask<T>,
    ) {
        let lang = self.query.settings.user_lang;
        let show_english = self.query.settings.show_english;

        let kanji_reading = self
            .query
            .form
            .as_kanji_reading()
            .and_then(|i| kanji::get_reading(i));

        search_task.set_result_filter(move |sentence| {
            if sentence.get_translation(lang, show_english).is_none() {
                return false;
            }

            if let Some(reading) = &kanji_reading {
                return kanji::sentence_matches(sentence, &reading);
            }

            true
        })
    }

    fn jp_reading(&self) -> String {
        let mut query_str = self.query.query_str.clone();

        if let Some(kanji_reading) = self.query.form.as_kanji_reading() {
            query_str = kanji_reading.literal.to_string();
        }

        query_str
    }
}
