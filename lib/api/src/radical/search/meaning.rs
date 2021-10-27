pub fn search(query: &str) -> Vec<char> {
    if query.len() < 2 {
        return vec![];
    }

    search::radical::search(query)
}
