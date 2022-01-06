use error::Error;
use strsim::jaro_winkler;
use types::jotoba::words::Word;
use utils::real_string_len;

use crate::engine::words::native::regex as regex_engine;
use crate::{engine::result::SearchResult, regex_query::RegexSQuery};

/// Searches the given `query` using regex search
pub fn search(
    query: RegexSQuery,
    limit: u32,
    offset: usize,
) -> Result<SearchResult<&'static Word>, Error> {
    let mut words = regex_engine::search(&query)?
        .into_iter()
        .map(|(word, src)| (word, word_score(word, src, &query)))
        .collect::<Vec<_>>();

    let len = words.len();

    words.sort_by(|a, b| a.1.cmp(&b.1).reverse());

    let words = words
        .into_iter()
        .skip(offset)
        .take(limit as usize)
        .collect::<Vec<_>>();

    Ok(SearchResult::from_items(words, len))
}

fn word_score(word: &Word, found_in: &str, query: &RegexSQuery) -> usize {
    let mut score = 100;

    if !word
        .reading
        .alternative
        .iter()
        .any(|i| i.reading == found_in)
    {
        score += 20;
    }

    if word.is_common() {
        score += 10;
    }

    if word.get_jlpt_lvl().is_some() {
        score += 5;
    }

    // Similarity to query
    let comp_query = query.get_chars().into_iter().collect::<String>();
    score += (jaro_winkler(found_in, &comp_query) * 100f64) as usize;

    // Show shorter words more on top
    score = score.wrapping_sub(real_string_len(&word.get_reading().reading));

    score
}
