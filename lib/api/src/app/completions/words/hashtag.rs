use index_framework::traits::{backend::Backend, storage::IndexStorage};
use std::ops::Deref;
use types::{api::app::completions::WordPair, jotoba::search::SearchTarget};

pub fn suggestions(query: &str, search_target: SearchTarget) -> Option<Vec<WordPair>> {
    if query.trim().is_empty() {
        return Some(empty(search_target));
    }

    let index = indexes::get_suggestions().hashtags();
    let res = index.ngram_search(query, &[search_target]);
    let max = res.first()?.1;

    let out: Vec<_> = res
        .into_iter()
        .filter(|i| i.1 >= max - 0.4)
        .map(|i| WordPair::new(i.0.tag.clone()))
        .collect();

    Some(out)
}

fn empty(search_target: SearchTarget) -> Vec<WordPair> {
    let start = std::time::Instant::now();
    let index = &indexes::get_suggestions().hashtags();
    let ngindex = index.index.deref();

    let mut out: Vec<_> = ngindex
        .storage()
        .iter()
        .map(|i| index.get(i.into_item() as usize).unwrap())
        .filter(|i| i.s_targets.contains(&search_target))
        .collect();

    out.sort_by(|a, b| a.freq.total_cmp(&b.freq).reverse());

    let res = out
        .into_iter()
        .take(10)
        .map(|i| WordPair::new(i.tag.clone()))
        .collect();
    println!("took: {:?}", start.elapsed());
    res
}
