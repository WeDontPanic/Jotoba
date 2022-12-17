use crate::{engine::words::native::Engine, word::order::native::NativeOrder};
use engine::{result::SearchResult, task::SearchTask};
use jp_utils::JapaneseExt;
use std::collections::HashSet;
use types::jotoba::words::Word;

/// Amount of words to return in a search for radicals
const WORD_LIMIT: usize = 3;

/// Search for radicals in words by a foreign query
pub struct Search<'a> {
    query: &'a str,
}

impl<'a> Search<'a> {
    #[inline]
    pub fn new(query: &'a str) -> Self {
        Self { query }
    }

    /// Does a kana word-search and returns some likely radicals for the given query
    #[inline]
    pub fn run(&self) -> HashSet<char> {
        let mut search_task = self.search_task();
        self.select_kanji(search_task.find())
    }

    #[inline]
    fn search_task(&self) -> SearchTask<'static, Engine> {
        SearchTask::new(&self.query)
            .with_limit(WORD_LIMIT)
            .with_threshold(0.8)
            .with_custom_order(NativeOrder::new(self.query.to_string()))
    }

    fn select_kanji(&self, res: SearchResult<&Word>) -> HashSet<char> {
        let kanji_retr = resources::get().kanji();
        res.into_iter()
            .map(|word| word.get_reading().reading.chars().filter(|i| i.is_kanji()))
            .flatten()
            .filter_map(|kanji| kanji_retr.by_literal(kanji).map(|i| &i.parts))
            .flatten()
            .copied()
            .take(10)
            .collect()
    }
}
