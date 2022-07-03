pub mod kanji;
pub mod result;
mod tag_only;

use super::query::Query;
use crate::{
    engine::{sentences::foreign, sentences::native, SearchEngine, SearchTask},
    query::{tags::Tag, Form, QueryLang},
};
use error::Error;
use result::SentenceResult;
use types::jotoba::{languages::Language, search::guess::Guess, sentences::Sentence};

/// Holds a query and can perform various types of a sentence search
pub struct Search<'a> {
    query: &'a Query,
}

impl<'a> Search<'a> {
    pub fn new(query: &'a Query) -> Self {
        Self { query }
    }

    /// Searches for sentences
    pub fn search(&self) -> Result<SentenceResult, Error> {
        let res = match self.query.form {
            Form::TagOnly => tag_only::search(self.query)?,
            Form::Sequence(seq) => self.by_sequence(seq),
            _ => self.normal(),
        };
        Ok(res)
    }

    /// Search by sequence id
    fn by_sequence(&self, seq: u32) -> SentenceResult {
        let sentence = match resources::get().sentences().by_id(seq) {
            Some(s) => s,
            None => return SentenceResult::default(),
        };

        let lang = self.query.get_search_lang();
        let show_english = self.query.settings.show_english();
        let sentence = match map_sentence_to_item(sentence, lang, show_english) {
            Some(s) => s,
            None => return SentenceResult::default(),
        };

        let hidden = self.query.has_tag(Tag::Hidden);
        SentenceResult {
            items: vec![sentence],
            len: 1,
            hidden,
        }
    }

    fn normal(&self) -> SentenceResult {
        if self.query.q_lang == QueryLang::Japanese {
            let query_str = self.jp_reading();
            self.get_result(self.japanese(&query_str))
        } else {
            self.get_result(self.foreign())
        }
    }

    fn foreign(&self) -> SearchTask<foreign::Engine> {
        let query_str = &self.query.query_str;

        let mut search_task =
            SearchTask::<foreign::Engine>::with_language(query_str, self.query.settings.user_lang)
                .limit(self.query.settings.page_size as usize)
                .offset(self.query.page_offset)
                .threshold(0.2);

        if self.query.settings.show_english && self.query.settings.user_lang != Language::English {
            search_task.add_language_query(&self.query.query_str, Language::English)
        }

        self.lang_filter(&mut search_task);
        self.sort_fn(query_str.to_string(), &mut search_task, false);

        search_task
    }

    fn japanese(&self, query_str: &'a str) -> SearchTask<'a, native::Engine> {
        let mut search_task = SearchTask::<native::Engine>::new(&query_str)
            .limit(self.query.settings.page_size as usize)
            .offset(self.query.page_offset)
            .threshold(0.2);

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
            let lang_filter = sentence.has_translation(lang)
                || (show_english && sentence.has_translation(Language::English));

            // No need to go further
            if !lang_filter {
                return false;
            }

            if let Some(reading) = &kanji_reading {
                return kanji::sentence_matches(sentence, &reading);
            }

            lang_filter
        })
    }

    fn get_result<T: SearchEngine<Output = &'static Sentence> + Send>(
        &self,
        search: SearchTask<T>,
    ) -> SentenceResult {
        let lang = self.query.settings.user_lang;
        let found = search.find();
        let len = found.len();
        let show_english = self.query.settings.show_english();

        let items = found
            .into_iter()
            .filter_map(|i| map_sentence_to_item(i, lang, show_english))
            .collect::<Vec<_>>();
        let hidden = self.query.has_tag(Tag::Hidden);
        SentenceResult { len, items, hidden }
    }

    fn jp_reading(&self) -> String {
        let mut query_str = self.query.query_str.clone();

        if let Some(kanji_reading) = self.query.form.as_kanji_reading() {
            query_str = kanji_reading.literal.to_string();
        }

        query_str
    }
}

pub fn map_sentence_to_item(
    sentence: &Sentence,
    lang: Language,
    show_english: bool,
) -> Option<result::Sentence> {
    result::Sentence::from_m_sentence(sentence.clone(), lang, show_english)
}

/// Guesses the amount of results a search would return with given `query`
pub fn guess_result(query: &Query) -> Option<Guess> {
    let search = Search::new(query);
    if query.q_lang == QueryLang::Japanese {
        let query_str = search.jp_reading();
        search.japanese(&query_str).estimate_result_count()
    } else {
        search.foreign().estimate_result_count()
    }
    .ok()
}
