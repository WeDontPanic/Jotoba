use std::time::Instant;

use error::Error;
use types::jotoba::words::Word;

use crate::engine::words::native::regex as regex_engine;
use crate::{engine::result::SearchResult, regex_query::RegexSQuery};

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
    let words = res
        .items
        .into_iter()
        .rev()
        .skip(offset)
        .take(limit as usize)
        .collect::<Vec<_>>();

    let res = SearchResult::from_items_ordered(words, res.item_len);
    println!("Regex search took: {:?}", start.elapsed());
    Ok(res)
}
