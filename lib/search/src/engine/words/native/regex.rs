use crate::{engine::utils::page_from_pqueue, query::regex::RegexSQuery};
use indexes::regex::RegexSearchIndex;
use intmap::int_set::IntSet;
use itertools::Itertools;
use order_struct::order_nh::OrderVal;
use priority_container::StableUniquePrioContainerMax;
use types::jotoba::words::Word;

/// Result of a regex search
pub struct RegexSearchResult {
    pub items: Vec<&'static Word>,
    // the total amount of items the search would return.
    // This value is most likely different than items.len()
    pub item_len: usize,
}

pub fn search<F>(query: &RegexSQuery, sort: F, limit: usize, offset: usize) -> RegexSearchResult
where
    F: Fn(&Word, &str) -> usize,
{
    let word_resources = resources::get().words();

    let queue_size = limit + offset;
    let mut out_queue = StableUniquePrioContainerMax::new_allocated(queue_size, queue_size);

    let index = indexes::get().word().regex();
    let possible_results = find_words(index, &query.get_chars());

    for seq_id in possible_results.into_iter().sorted() {
        let word = word_resources.by_sequence(seq_id).unwrap();

        let item_iter = word
            .reading_iter(true)
            .filter_map(|i| query.matches(&i.reading).then(|| (word, &i.reading)))
            .map(|(word, reading)| {
                let order = sort(word, reading);
                OrderVal::new(word, order)
            });

        out_queue.extend(item_iter);
    }

    let item_len = out_queue.total_pushed();

    let items: Vec<_> = page_from_pqueue(limit, offset, out_queue)
        .into_iter()
        .map(|i| i.into_inner())
        .collect();

    RegexSearchResult { items, item_len }
}

/// Get all indexed words using characters in `chars`
fn find_words(index: &RegexSearchIndex, chars: &[char]) -> IntSet {
    if chars.is_empty() {
        return IntSet::new();
    }

    let mut out = IntSet::new();

    // Add words of first character to `out`
    let mut chars_iter = chars.iter();

    // We want to fill `out` with some values.
    loop {
        let first = match chars_iter.next() {
            Some(f) => f,
            None => break,
        };

        if let Some(v) = index.get_words_with(*first) {
            out.reserve(v.len());
            out.extend(v.iter().copied());
            // exit first found character
            break;
        }
    }

    for v in chars_iter.filter_map(|c| index.get_words_with(*c)) {
        out.retain(|i| v.contains(&i));
        if out.is_empty() {
            return IntSet::new();
        }
    }

    out
}
