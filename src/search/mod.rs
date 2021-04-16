#![allow(dead_code)]

pub mod everything;
pub mod result;
mod result_order;
pub mod word;

pub use self::result::Item as ResultItem;

/// How db entries should be matched with
/// the query in order to be valid as result
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum SearchMode {
    Exact,
    Variable,
    LeftVariable,
    RightVariable,
}

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
