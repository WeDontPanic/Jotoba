use engine::task::SearchTask;
use jp_utils::JapaneseExt;

use crate::{
    engine::words::native::Engine,
    query::Query,
    word::{filter::WordFilter, order::native::NativeOrder},
};

/// Helper for creating SearchTask for foreign queries
pub struct NativeSearch<'a> {
    query: &'a Query,
    query_str: &'a str,
    cust_original: Option<&'a str>,
    threshold: f32,
}

impl<'a> NativeSearch<'a> {
    #[inline]
    pub(crate) fn new(query: &'a Query, query_str: &'a str) -> Self {
        // Kanji queries are shorter so we need a lower threshold to not filter too many different words for short queries
        let kana_count: usize = query_str.chars().filter(|i| i.is_kana()).count();
        let kanji_count: usize = query_str.chars().filter(|i| i.is_kanji()).count();
        let kanji_query = kanji_count >= (kana_count * 2);
        let threshold = if kanji_query || (kanji_count + kana_count < 5) {
            0.15
        } else {
            0.3
        };

        Self {
            query,
            query_str,
            cust_original: None,
            threshold,
        }
    }

    pub fn with_custom_original_query(mut self, query: &'a str) -> Self {
        self.cust_original = Some(query);
        self
    }

    pub fn with_threshold(mut self, threshold: f32) -> Self {
        self.threshold = threshold;
        self
    }

    pub fn task(&self) -> SearchTask<'static, Engine> {
        let filter = WordFilter::new(self.query.clone());
        let original_query = self.original_query().to_string();

        SearchTask::new(self.query_str)
            .with_custom_order(NativeOrder::new(original_query))
            .with_result_filter(move |item| !filter.filter_word(*item))
            .with_threshold(self.threshold)
    }

    #[inline]
    pub fn original_query(&self) -> &str {
        self.cust_original
            .as_ref()
            .unwrap_or(&self.query.raw_query.as_str())
    }
}
