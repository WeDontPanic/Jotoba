pub mod kanji;
pub mod name;
pub mod word;

use self::{kanji::KanjiRetrieve, name::NameRetrieve, word::WordRetrieve};
use super::{kanji::Kanji, names::Name, words::Word, Resources};
use std::collections::HashMap;

type WordStorage = HashMap<u32, Word>;
type NameStorage = HashMap<u32, Name>;
type KanjiStorage = HashMap<char, Kanji>;

#[derive(Default)]
pub struct ResourceStorage {
    words: WordStorage,
    names: NameStorage,
    kanji: KanjiStorage,
}

impl ResourceStorage {
    /// Create a new `ResourceStorage` by `Resources`
    #[inline]
    pub fn new(resources: Resources) -> Self {
        Self {
            words: build_words(resources.words),
            names: build_names(resources.names),
            kanji: build_kanji(resources.kanji),
        }
    }

    /// Returns a `WordRetrieve` which can be used to retrieve words from the `ResourceStorage`
    #[inline]
    pub fn words(&self) -> WordRetrieve<'_> {
        WordRetrieve::new(self)
    }

    /// Returns a `WordRetrieve` which can be used to retrieve names from the `ResourceStorage`
    #[inline]
    pub fn names(&self) -> NameRetrieve<'_> {
        NameRetrieve::new(self)
    }

    /// Returns a `KanjiRetrieve` which can be used to retrieve kanji from the `ResourceStorage`
    #[inline]
    pub fn kanji(&self) -> KanjiRetrieve<'_> {
        KanjiRetrieve::new(self)
    }
}

#[inline]
fn build_words(words: Vec<Word>) -> WordStorage {
    words.into_iter().map(|i| (i.sequence, i)).collect()
}

#[inline]
fn build_names(names: Vec<Name>) -> NameStorage {
    names.into_iter().map(|i| (i.sequence, i)).collect()
}

#[inline]
fn build_kanji(kanji: Vec<Kanji>) -> KanjiStorage {
    kanji.into_iter().map(|i| (i.literal, i)).collect()
}

impl From<Resources> for ResourceStorage {
    #[inline]
    fn from(resources: Resources) -> Self {
        ResourceStorage::new(resources)
    }
}
