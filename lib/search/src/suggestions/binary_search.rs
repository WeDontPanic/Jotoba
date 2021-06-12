use std::cmp::Ordering;

use super::{store_item::Item, text_store::TextStore};

/// Represents a binary search adjusted for starts_with search
pub(crate) struct Search<'a, T: TextStore> {
    query: String,
    text_store: &'a T,
}

impl<'a, T: TextStore> Search<'a, T> {
    pub(crate) fn new(query: String, text_store: &'a T) -> Self {
        Self { query, text_store }
    }

    /// Finds first matching item
    pub(crate) fn find_first(&self) -> Option<usize> {
        // Find using binary search. If multiple results found (which is very likely the case in
        // our implementation), a random item of the matching ones will be found
        let random_index = self
            .text_store
            .binary_search_by(|a| my_cmp(a.get_text(), &self.query));

        // None found
        if random_index >= self.text_store.len() {
            return None;
        }

        // Since its a random item, find first matching item
        let mut pos = random_index;
        loop {
            if pos == 0 {
                break;
            }
            let prev_pos = pos - 1;
            let prev_item = self.text_store.get_at(prev_pos);
            if my_cmp(prev_item.get_text(), &self.query) == Ordering::Equal {
                if pos == 0 {
                    break;
                }
                pos = prev_pos;
            } else {
                break;
            }
        }
        Some(pos)
    }

    /// Returns an iterator over each matching result
    pub(crate) fn search(self) -> impl Iterator<Item = &'a T::Item> {
        let first_item = self.find_first();

        let mut item_pos = 0;
        std::iter::from_fn(move || {
            let first_item = first_item?;
            let curr_item_pos = first_item + item_pos;

            if curr_item_pos >= self.text_store.len() {
                return None;
            }

            let item = self.text_store.get_at(curr_item_pos);
            if my_cmp(item.get_text(), &self.query) == Ordering::Equal {
                item_pos += 1;
                return Some(item);
            }

            None
        })
    }
}

/// Custom comparing function to match starts_with items
fn my_cmp(a: &str, b: &str) -> Ordering {
    if a.starts_with(b) {
        Ordering::Equal
    } else {
        a.cmp(b)
    }
}
