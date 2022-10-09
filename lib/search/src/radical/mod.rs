/// Radical word search
pub mod word;

use std::collections::HashSet;

/// Finds radicals by its meanings
pub fn search(query: &str) -> HashSet<char> {
    crate::engine::radical::find(query)
        .into_iter()
        .map(|j| j.literal)
        .collect()
}
