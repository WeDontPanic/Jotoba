use serde::Serialize;

use crate::jotoba::{pagination::page::Page, search::help::SearchHelp};

pub mod kanji;
pub mod names;
pub mod sentences;
pub mod words;

#[derive(Serialize)]
pub struct Response<T: Serialize> {
    #[serde(flatten)]
    inner: Page<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    search_help: Option<SearchHelp>,
}

impl<T: Serialize> Response<T> {
    pub fn new(inner: Page<T>) -> Self {
        Self {
            inner,
            search_help: None,
        }
    }

    pub fn with_help(inner: Page<T>, search_help: SearchHelp) -> Self {
        Self {
            inner,
            search_help: Some(search_help),
        }
    }

    pub fn with_help_fn<S>(inner: Page<T>, help_fn: S) -> Self
    where
        S: Fn(&Page<T>) -> Option<SearchHelp>,
    {
        Self {
            search_help: help_fn(&inner),
            inner,
        }
    }

    pub fn set_search_help(&mut self, search_help: SearchHelp) -> &mut Self {
        self.search_help = Some(search_help);
        self
    }
}
