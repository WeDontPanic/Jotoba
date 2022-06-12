use serde::{Deserialize, Serialize};

use crate::jotoba::languages::Language;

use super::{guess::Guess, QueryType};

/// Structure containing information for better search help in case no item was
/// found in a search
#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct SearchHelp {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub words: Option<Guess>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub names: Option<Guess>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sentences: Option<Guess>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kanji: Option<Guess>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub other_langs: Vec<Language>,
}

impl SearchHelp {
    pub fn new(
        words: Option<Guess>,
        names: Option<Guess>,
        sentences: Option<Guess>,
        kanji: Option<Guess>,
        other_langs: Vec<Language>,
    ) -> Self {
        Self {
            words,
            names,
            sentences,
            kanji,
            other_langs,
        }
    }

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

    pub fn iter_langs(&self) -> impl Iterator<Item = (Language, &'static str)> + '_ {
        self.other_langs
            .iter()
            .map(|lang| (*lang, lang.to_query_format()))
    }
}
