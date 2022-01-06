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
    let mut words = regex_engine::search(&query)?
        .into_iter()
        .map(|(word, src)| (word, regex_order(word, src, &query)))
        .collect::<Vec<_>>();

    let len = words.len();

    // Already sort them here so we can take only those to display
    words.sort_by(|a, b| a.1.cmp(&b.1).reverse());

    // Select words to display
    let words = words
        .into_iter()
        .skip(offset)
        .take(limit as usize)
        .collect::<Vec<_>>();

    Ok(SearchResult::from_items_ordered(words, len))
}
