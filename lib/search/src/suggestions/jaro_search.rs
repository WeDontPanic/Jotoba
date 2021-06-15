use std::{
    cmp::min,
    pin::Pin,
    task::{Context, Poll},
};

use super::{store_item::Item, text_store::TextStore};
use futures::Future;

/// Represents a binary search adjusted for starts_with search
pub(crate) struct Search<'a, T: TextStore> {
    query: &'a str,
    text_store: &'a T,
    last_pos: usize,
    len_limit: usize,
    eudex_hash: eudex::Hash,
    query_len: usize,
}

impl<'a, T: TextStore> Search<'a, T> {
    pub(crate) fn new(query: &'a str, text_store: &'a T, len_limit: usize) -> Self {
        Self {
            query,
            text_store,
            last_pos: 0,
            len_limit,
            eudex_hash: eudex::Hash::new(query),
            query_len: query.len(),
        }
    }

    /// Returns an iterator over each result. Calling `search` without using the result does
    /// nothing
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

    /// Returns Some(&'a T::Item) if the item at position [`i`] matches the query using eudex hash
    fn match_item(&self, i: usize) -> Option<&'a T::Item> {
        let item = self.text_store.get_at(i);
        let item_text = item.get_text();

        // Filter out impossible/unlike matches
        if self.query_len > item_text.len() || self.query_len + self.len_limit < item_text.len() {
            return None;
        }

        (self.eudex_hash - item.get_hash()).similar().then(|| item)
    }
}

/// AsyncSearch implementation for `Search`
pub(crate) struct AsyncSearch<'a, T: TextStore> {
    search: Search<'a, T>,
    result: Vec<&'a T::Item>,
}

impl<'a, T: TextStore> AsyncSearch<'a, T> {
    pub(crate) fn new(search: Search<'a, T>) -> Self {
        Self {
            search,
            result: Vec::new(),
        }
    }
}

impl<'a, T: TextStore> Future for AsyncSearch<'a, T> {
    type Output = Vec<&'a T::Item>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let start = self.search.last_pos;

        if start >= self.search.text_store.len() {
            // We're done here
            return Poll::Ready(std::mem::take(&mut self.result));
        }

        let end = min(self.search.text_store.len(), start + 2500);

        for i in start..end {
            if let Some(item) = self.search.match_item(i) {
                self.as_mut().result.push(item);
            }
        }
        self.as_mut().search.last_pos = end;

        cx.waker().wake_by_ref();

        return Poll::Pending;
    }
}

impl<'a, T: TextStore> From<Search<'a, T>> for AsyncSearch<'a, T> {
    fn from(search: Search<'a, T>) -> AsyncSearch<'a, T> {
        AsyncSearch::new(search)
    }
}
