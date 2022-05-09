use error::Error;
use order_struct::OrderVal;
use priority_container::PrioContainerMax;
use types::jotoba::words::Word;

use crate::regex_query::RegexSQuery;

use super::regex_index;

pub struct RegexSearchResult {
    pub items: Vec<&'static Word>,
    // the total amount of items the search would return.
    // This value is most likely different than items.len()
    pub item_len: usize,
}

pub fn search<F>(query: &RegexSQuery, sort: F, limit: usize) -> Result<RegexSearchResult, Error>
where
    F: Fn(&Word, &str) -> usize,
{
    let possible_results = regex_index::get().find(&query.get_chars());

    let mut out_queue = PrioContainerMax::new(limit);

    let word_resources = resources::get().words();

    let mut len: usize = 0;
    for seq_id in possible_results {
        let word = match word_resources.by_sequence(seq_id) {
            Some(w) => w,
            None => continue,
        };

        let item_iter = word
            .reading_iter(true)
            .filter_map(|i| query.matches(&i.reading).then(|| (word, &i.reading)))
            .map(|(word, reading)| {
                let order = sort(word, reading);
                OrderVal::new(word, order)
            })
            .inspect(|_| len += 1);
        out_queue.extend(item_iter);
    }

    let items: Vec<_> = out_queue.into_iter().map(|i| i.0.into_inner()).collect();
    Ok(RegexSearchResult {
        items,
        item_len: len,
    })
}
