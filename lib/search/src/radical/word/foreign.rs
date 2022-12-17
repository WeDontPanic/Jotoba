use std::collections::HashSet;

use crate::{engine::words::foreign::Engine, word::order::foreign::ForeignOrder};
use engine::{result::SearchResult, task::SearchTask};
use jp_utils::JapaneseExt;
use types::jotoba::{languages::Language, words::Word};

/// Amount of words to return in a search for radicals
const WORD_LIMIT: usize = 3;

/// Search for radicals in words by a foreign query
pub struct Search<'a> {
    query: &'a str,
    lang: Language,
}

impl<'a> Search<'a> {
    #[inline]
    pub fn new(query: &'a str, lang: Language) -> Self {
        Self { query, lang }
    }

    /// Does a kana word-search and returns some likely radicals for the given query
    #[inline]
    pub fn run(&self) -> HashSet<char> {
        let mut search_task = self.search_task();
        self.select_kanji(search_task.find())
    }

    #[inline]
    fn search_task(&self) -> SearchTask<'static, Engine> {
        SearchTask::with_language(&self.query, self.lang)
            .with_custom_order(ForeignOrder::new())
            .with_limit(WORD_LIMIT)
    }

    fn select_kanji(&self, res: SearchResult<&Word>) -> HashSet<char> {
        let kanji_retr = resources::get().kanji();
        res.into_iter()
            .filter(|word| word.get_reading().reading == self.query)
            .map(|word| word.get_reading().reading.chars().filter(|i| i.is_kanji()))
            .flatten()
            .filter_map(|kanji| kanji_retr.by_literal(kanji).map(|i| &i.parts))
            .flatten()
            .copied()
            .take(10)
            .collect()
    }
}
