use error::Error;
use types::jotoba::words::Word;

use crate::regex_query::RegexSQuery;

use super::regex_index;

pub fn search(query: &RegexSQuery) -> Result<Vec<(&'static Word, &'static String)>, Error> {
    let index = regex_index::get();
    let word_resources = resources::get().words();

    let possible_results = index.find(&query.get_chars());

    let mut out = Vec::with_capacity(possible_results.len());

    for result in possible_results {
        if query.matches(&result.text) {
            if let Some(word) = word_resources.by_sequence(result.seq_id) {
                out.push((word, &result.text));
            }
        }
    }

    // Since we're using a HashSet, we need the results to be in a constant order, or the order
    // will be slightly different in each search
    out.sort_by(|a, b| a.0.sequence.cmp(&b.0.sequence));

    Ok(out)
}
