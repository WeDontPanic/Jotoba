pub mod engine;
pub mod executor;
pub mod kanji;
pub mod name;
pub mod query;
pub mod radical;
pub mod result;
pub mod sentence;
pub mod word;

pub use executor::SearchExecutor;

use query::Query;
use types::jotoba::search::{help::SearchHelp, SearchTarget};

/// How string items should be matched with each other
#[derive(Clone, Copy, Debug)]
pub enum SearchMode {
    Exact,
    Variable,
    RightVariable,
    LeftVariable,
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

    pub fn ordered_iter() -> impl Iterator<Item = &'static SearchMode> {
        [
            SearchMode::Exact,
            SearchMode::Variable,
            SearchMode::RightVariable,
            SearchMode::LeftVariable,
        ]
        .iter()
    }
}

/// Build a [`SearchHelp`] in for cases without any search results
pub fn build_help(querytype: SearchTarget, query: &Query) -> Option<SearchHelp> {
    let mut help = SearchHelp::default();

    for qt in SearchTarget::iterate().filter(|i| *i != querytype) {
        match qt {
            SearchTarget::Kanji => help.kanji = kanji::guess_result(query),
            SearchTarget::Sentences => {
                help.sentences = SearchExecutor::new(sentence::Search::new(query)).guess()
            }
            SearchTarget::Names => {
                help.names = SearchExecutor::new(name::Search::new(query)).guess()
            }
            SearchTarget::Words => {
                help.words = SearchExecutor::new(word::Search::new(query)).guess()
            }
        }
    }

    if querytype == SearchTarget::Words {
        help.other_langs = word::guess_inp_language(query);
    }

    (!help.is_empty()).then(|| help)
}
