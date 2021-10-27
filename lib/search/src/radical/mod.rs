/// Finds radicals by its meanings
pub fn search(query: &str) -> Vec<char> {
    crate::engine::radical::find(query)
        .into_iter()
        .map(|j| j.literal)
        .collect()
}
