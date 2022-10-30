use types::jotoba::kanji::radical::SearchRadicalInfo;

/// Finds Radicals by its meaning(s). If `query_str` was not found
/// as meaning of an radical, similar meanings are being searched
/// and added to the result
pub fn find(query_str: &str) -> Vec<&'static SearchRadicalInfo> {
    let mut queries = vec![query_str];

    let meaning_index = indexes::get().radical().meaning_index();

    if !meaning_index.has_term(query_str) {
        add_similar(query_str, &mut queries);
    }

    queries
        .into_iter()
        .filter_map(|term| meaning_index.get(term))
        .flatten()
        .take(5)
        .collect()
}

/// Adds meanings of radicals with similar meaning as `query_str` to `out`
fn add_similar(query_str: &str, out: &mut Vec<&str>) {
    let meaning_index = indexes::get().radical().meaning_index();

    // Search term in meanings
    let mut found = meaning_index.term_tree.find(&query_str.to_string(), 2);

    // Show more similar terms above
    found.sort_by(|a, b| a.1.cmp(&b.1).reverse());

    // Assign `queries` to a new vec because it can only contain in index existing terms
    out.extend(found.into_iter().take(3).map(|i| i.0.as_str()));
}
