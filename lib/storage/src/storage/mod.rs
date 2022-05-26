pub mod kanji;
pub mod name;
pub mod sentence;
pub mod word;

use self::{kanji::KanjiStorage, name::NameStorage, sentence::SentenceStorage, word::WordStorage};
use serde::{Deserialize, Serialize};

/// Storage holding all data of Jotoba
#[derive(Serialize, Deserialize, Default)]
pub struct ResourceStorage {
    words: Option<WordStorage>,
    kanji: Option<KanjiStorage>,
    names: Option<NameStorage>,
    sentences: Option<SentenceStorage>,
}

impl ResourceStorage {
    /// Create a new empty `ResourceStorage`
    pub fn new() -> Self {
        Self::default()
    }

    /// Get a reference to the resource storage's words.
    #[inline(always)]
    pub fn words(&self) -> &WordStorage {
        unsafe { self.words.as_ref().unwrap_unchecked() }
    }

    /// Get a reference to the resource storage's kanji.
    #[inline(always)]
    pub fn kanji(&self) -> &KanjiStorage {
        unsafe { self.kanji.as_ref().unwrap_unchecked() }
    }

    /// Get a reference to the resource storage's names.
    #[inline(always)]
    pub fn names(&self) -> &NameStorage {
        unsafe { self.names.as_ref().unwrap_unchecked() }
    }

    /// Get a reference to the resource storage's sentences.
    #[inline(always)]
    pub fn sentences(&self) -> &SentenceStorage {
        unsafe { self.sentences.as_ref().unwrap_unchecked() }
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
