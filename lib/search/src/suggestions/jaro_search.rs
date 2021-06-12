use strsim::jaro_winkler;

use super::{store_item::Item, text_store::TextStore};

/// Represents a binary search adjusted for starts_with search
pub(crate) struct Search<'a, T: TextStore> {
    query: &'a str,
    text_store: &'a T,
    last_pos: usize,
    len_limit: usize,
}

impl<'a, T: TextStore> Search<'a, T> {
    pub(crate) fn new(query: &'a str, text_store: &'a T, len_limit: usize) -> Self {
        Self {
            query,
            text_store,
            last_pos: 0,
            len_limit,
        }
    }

    pub(crate) fn search(mut self) -> impl Iterator<Item = &'a T::Item> {
        std::iter::from_fn(move || {
            let start = self.last_pos;

            if start >= self.text_store.len() {
                // We're done here
                return None;
            }

            for i in start..self.text_store.len() {
                if let Some(item) = self.match_item(i) {
                    self.last_pos += 1;
                    return Some(item);
                }
                self.last_pos += 1;
            }

            None
        })
    }

    fn match_item(&self, i: usize) -> Option<&'a T::Item> {
        let item = self.text_store.get_at(i);
        let item_text = item.get_text();

        // Filter out impossible/unlike matches
        if self.query.len() > item_text.len() || self.query.len() + self.len_limit < item_text.len()
        {
            return None;
        }

        let lvst = self.normalized_levenshtein(item_text);
        if lvst > 0.8 {
            Some(item)
        } else {
            None
        }
    }

    fn normalized_levenshtein(&self, s1: &str) -> f64 {
        jaro_winkler(&self.query.to_lowercase(), &s1.to_lowercase())
    }
}
