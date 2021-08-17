pub mod kanji;
pub mod name;
pub mod suggestion;
pub mod word;

use crate::parse::jmdict::languages::Language;

use self::{
    kanji::KanjiRetrieve,
    name::NameRetrieve,
    suggestion::{provider::SuggestionProvider, SuggestionDictionary},
    word::WordRetrieve,
};
use super::{kanji::Kanji, names::Name, words::Word, DictResources};
use std::collections::HashMap;

type WordStorage = HashMap<u32, Word>;
type NameStorage = HashMap<u32, Name>;
type KanjiStorage = HashMap<char, Kanji>;

#[derive(Default)]
pub struct ResourceStorage {
    dict_data: DictionaryData,
    suggestions: Option<SuggestionData>,
}

#[derive(Default)]
struct DictionaryData {
    words: WordStorage,
    names: NameStorage,
    kanji: KanjiStorage,
}

#[derive(Default)]
pub(super) struct SuggestionData {
    foregin: HashMap<Language, SuggestionDictionary>,
    japanese: Option<SuggestionDictionary>,
}

impl DictionaryData {
    #[inline]
    fn new(words: WordStorage, names: NameStorage, kanji: KanjiStorage) -> Self {
        Self {
            words,
            names,
            kanji,
        }
    }
}

impl SuggestionData {
    #[inline]
    pub(super) fn new() -> Self {
        Self::default()
    }
}

impl ResourceStorage {
    /// Create a new `ResourceStorage` by `Resources`
    #[inline]
    pub(super) fn new(resources: DictResources, suggestions: Option<SuggestionData>) -> Self {
        let words = build_words(resources.words);
        let names = build_names(resources.names);
        let kanji = build_kanji(resources.kanji);
        let dict_data = DictionaryData::new(words, names, kanji);

        Self {
            dict_data,
            suggestions,
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

    /// Returns a `SuggestionProvider` which can be used to retrieve suggestions from the `ResourceStorage`
    #[inline]
    pub fn suggestions(&self) -> SuggestionProvider<'_> {
        SuggestionProvider::new(self)
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
