pub mod kanji;
pub mod name;
pub mod sentence;
pub mod word;

use crate::retrieve::{
    kanji::KanjiRetrieve, name::NameRetrieve, sentence::SentenceRetrieve, word::WordRetrieve,
};

use self::{kanji::KanjiStorage, name::NameStorage, sentence::SentenceStorage, word::WordStorage};
use serde::{Deserialize, Serialize};

/// Storage holding all data of Jotoba
#[derive(Serialize, Deserialize, Default)]
pub struct ResourceStorage {
    pub words: Option<WordStorage>,
    pub kanji: Option<KanjiStorage>,
    pub names: Option<NameStorage>,
    pub sentences: Option<SentenceStorage>,
}

impl ResourceStorage {
    /// Create a new empty `ResourceStorage`
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns `true` if all necessary data is present
    #[inline(always)]
    pub fn check(&self) -> bool {
        self.words.is_some()
            && self.kanji.is_some()
            && self.names.is_some()
            && self.sentences.is_some()
    }
}

// Retrieve functions
// `ResourceStorage::check` is supposed to be called at the begininng to ensure
// those fields are not unset
impl ResourceStorage {
    /// Get a reference to the resource storage's words.
    #[inline(always)]
    pub fn words(&self) -> WordRetrieve {
        let word_store = unsafe { self.words.as_ref().unwrap_unchecked() };
        WordRetrieve::new(word_store)
    }

    /// Get a reference to the resource storage's kanji.
    #[inline(always)]
    pub fn kanji(&self) -> KanjiRetrieve {
        let kanji_store = unsafe { self.kanji.as_ref().unwrap_unchecked() };
        KanjiRetrieve::new(kanji_store)
    }

    /// Get a reference to the resource storage's names.
    #[inline(always)]
    pub fn names(&self) -> NameRetrieve {
        let name_store = unsafe { self.names.as_ref().unwrap_unchecked() };
        NameRetrieve::new(name_store)
    }

    /// Get a reference to the resource storage's sentences.
    #[inline(always)]
    pub fn sentences(&self) -> SentenceRetrieve {
        let sentence_store = unsafe { self.sentences.as_ref().unwrap_unchecked() };
        SentenceRetrieve::new(sentence_store)
    }
}
