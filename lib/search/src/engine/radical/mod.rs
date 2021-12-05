use types::jotoba::kanji::SearchRadicalInfo;

pub mod index;

/// Finds Radicals by its meaning(s)
pub fn find(inp_query: &str) -> Vec<&'static SearchRadicalInfo> {
    let mut queries = vec![inp_query];
    let index = index::get_index();

    if !index.has_term(inp_query) {
        let mut found = index.term_tree.find(&inp_query.to_string(), 2);
        found.sort_by(|a, b| a.1.cmp(&b.1));

        // Assign `queries` to a new vec because it can only contain in index existing terms
        queries = found.into_iter().take(3).map(|i| i.0.as_str()).collect();
    }

    queries
        .into_iter()
        .filter_map(|term| index.get(term))
        .flatten()
        .take(5)
        .collect()
}
