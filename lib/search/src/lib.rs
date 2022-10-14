pub mod engine;
pub mod executor;
pub mod kanji;
pub mod name;
pub mod query;
pub mod radical;
pub mod sentence;
pub mod word;

pub use executor::SearchExecutor;

use query::Query;
use types::jotoba::search::{help::SearchHelp, SearchTarget};

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
        //help.other_langs = word::guess_inp_language(query);
    }

    (!help.is_empty()).then(|| help)
}
