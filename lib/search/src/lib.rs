#![allow(dead_code)]

#[macro_use]
extern crate diesel;

use models::search_mode::SearchMode;

pub mod kanji;
pub mod name;
pub mod suggestions;
pub mod query;
pub mod query_parser;
pub mod search_order;
pub mod sentence;
pub mod word;

/// Predefines data, required for
/// each type of search
#[derive(Clone, PartialEq, Debug)]
pub struct Search<'a> {
    pub query: &'a str,
    pub limit: u16,
    pub mode: SearchMode,
}

impl<'a> Search<'a> {
    pub fn new(query: &'a str, mode: SearchMode) -> Self {
        Self {
            query,
            limit: 0,
            mode,
        }
    }

    /// Add a limit to the search
    pub fn with_limit(&mut self, limit: u16) -> &mut Self {
        self.limit = limit;
        self
    }
}
