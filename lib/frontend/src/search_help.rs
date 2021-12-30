use search::engine::guess::Guess;
use types::jotoba::{languages::Language as ResLanguage, search::QueryType};

/// Structure containing information for better search help in case no item was
/// found in a search
#[derive(Clone, Default, Debug)]
pub struct SearchHelp {
    pub words: Option<Guess>,
    pub names: Option<Guess>,
    pub sentences: Option<Guess>,
    pub kanji: Option<Guess>,
    pub other_langs: Vec<ResLanguage>,
}

impl SearchHelp {
    /// Returns `true` if `SearchHelp` is not helpful at all (empty)
    pub fn is_empty(&self) -> bool {
        self.iter_items().next().is_none()
    }

    /// Returns an iterator over all (QueryType, Guess) pairs that have a value
    pub fn iter_items(&self) -> impl Iterator<Item = (QueryType, Guess)> {
        let types = &[
            (self.words, QueryType::Words),
            (self.names, QueryType::Names),
            (self.sentences, QueryType::Sentences),
            (self.kanji, QueryType::Kanji),
        ];

        types
            .iter()
            .filter_map(|i| i.0.is_some().then(|| (i.1, i.0.unwrap())))
            .filter(|i| i.1.value != 0)
            .collect::<Vec<_>>()
            .into_iter()
    }

    pub fn iter_langs(&self) -> impl Iterator<Item = (ResLanguage, &'static str)> + '_ {
        self.other_langs
            .iter()
            .map(|lang| (*lang, lang.to_query_format()))
    }
}
