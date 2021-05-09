#![allow(dead_code)]

pub mod kanji;
pub mod name;
pub mod query;
pub mod query_parser;
pub mod search_order;
pub mod sentence;
pub mod word;

sql_function! {
    fn lower(a: diesel::types::VarChar) -> diesel::types::VarChar;
}

/// How db entries should be matched with
/// the query in order to be valid as result
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum SearchMode {
    Exact,
    Variable,
    LeftVariable,
    RightVariable,
}

impl SearchMode {
    /// Compares a string based on the mode and case
    pub fn str_eq<S: AsRef<str>>(&self, a: S, b: S, ign_case: bool) -> bool {
        let (a, b) = if ign_case {
            (a.as_ref().to_lowercase(), b.as_ref().to_lowercase())
        } else {
            (a.as_ref().to_owned(), b.as_ref().to_owned())
        };

        match *self {
            SearchMode::Exact => a == b,
            SearchMode::Variable => a.contains(&b),
            SearchMode::LeftVariable => a.starts_with(&b),
            SearchMode::RightVariable => a.ends_with(&b),
        }
    }

    /// Returns a string which can be placed inside a like query
    pub fn to_like<S: AsRef<str>>(&self, a: S) -> String {
        match self {
            SearchMode::Exact => a.as_ref().to_owned(),
            SearchMode::Variable => format!("%{}%", a.as_ref()),
            SearchMode::LeftVariable => format!("%{}", a.as_ref()),
            SearchMode::RightVariable => format!("{}%", a.as_ref()),
        }
    }
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
