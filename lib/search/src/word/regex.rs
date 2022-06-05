use crate::{
    engine::{result::SearchResult, result_item::ResultItem, words::native::regex as regex_engine},
    query::regex::RegexSQuery,
};
use error::Error;
use log::debug;
use std::time::Instant;
use types::jotoba::words::Word;

use super::order::regex_order;

/// Searches the given `query` using regex search
pub fn search(
    query: RegexSQuery,
    limit: u32,
    offset: usize,
) -> Result<SearchResult<&'static Word>, Error> {
    let start = Instant::now();

    let res = regex_engine::search(
        &query,
        |word, reading| regex_order(word, reading, &query),
        limit as usize + offset,
    )?;

    // Select words to display
    let words: Vec<ResultItem<_>> = res
        .items
        .into_iter()
        .map(|i| i.into())
        .rev()
        .skip(offset)
        .take(limit as usize)
        .collect::<Vec<_>>();

    let res = SearchResult::new(words, res.item_len);
    debug!("Regex search took: {:?}", start.elapsed());
    Ok(res)
}
