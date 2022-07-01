use query::Query;
use types::jotoba::search::{help::SearchHelp, SearchTarget};

pub mod engine;
pub mod kanji;
pub mod name;
pub mod query;
pub mod radical;
pub mod sentence;
pub mod word;

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
            SearchTarget::Sentences => help.sentences = sentence::guess_result(query),
            SearchTarget::Names => help.names = name::guess_result(query),
            SearchTarget::Words => help.words = word::guess_result(query),
        }
    }

    if querytype == SearchTarget::Words {
        help.other_langs = word::guess_inp_language(query);
    }

    (!help.is_empty()).then(|| help)
}
