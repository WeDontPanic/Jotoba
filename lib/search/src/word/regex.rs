use crate::{
    engine::{result::SearchResult, result_item::ResultItem, words::native::regex as regex_engine},
    query::regex::RegexSQuery,
};
use log::debug;
use std::time::Instant;
use types::jotoba::words::Word;

use super::order::regex_order;

/// Searches the given `query` using regex search
pub fn search(query: RegexSQuery, limit: usize, offset: usize) -> SearchResult<&'static Word> {
    let start = Instant::now();

    let res = regex_engine::search(
        &query,
        |word, reading| regex_order(word, reading, &query),
        limit,
        offset,
    );

    // Select words to display
    let words: Vec<ResultItem<_>> = res.items.into_iter().map(|i| i.into()).collect();

    debug!("Regex search took: {:?}", start.elapsed());
    SearchResult::new(words, res.item_len)
}
