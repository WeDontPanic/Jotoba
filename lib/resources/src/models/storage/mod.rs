pub mod kanji;
pub mod name;
pub mod sentence;
pub mod suggestion;
pub mod word;

use intmap::IntMap;

use crate::parse::jmdict::languages::Language;
use serde::{Deserialize, Serialize};

use self::{
    kanji::KanjiRetrieve,
    name::NameRetrieve,
    sentence::SentenceRetrieve,
    suggestion::{provider::SuggestionProvider, SuggestionDictionary},
    word::WordRetrieve,
};
use super::{
    kanji::{DetailedRadical, Kanji},
    names::Name,
    sentences::Sentence,
    suggestions::{foreign_words::ForeignSuggestion, native_words::NativeSuggestion},
    words::Word,
    DictResources,
};
use std::collections::HashMap;

pub type WordStorage = IntMap<Word>;
type NameStorage = IntMap<Name>;
type KanjiStorage = HashMap<char, Kanji>;
pub(super) type RadicalStorage = HashMap<char, Vec<char>>;

/// A dictionary of words, names, kanji, and radicals. This is the main data structure for the dictionary.
#[derive(Default)]
pub struct ResourceStorage {
    pub dict_data: DictionaryData,
    suggestions: Option<SuggestionData>,
}

/// Contains all core data for the dictionary. This is the data structure for the dictionary functionality to work properly.
#[derive(Default)]
pub struct DictionaryData {
    word_data: WordData,
    names: NameStorage,
    kanji: KanjiData,
    rad_map: RadicalStorage,
    sentences: SentenceStorage,
    radicals: HashMap<char, DetailedRadical>,
}

/// Represents the dictionary word data
#[derive(Default)]
pub struct WordData {
    words: WordStorage,
    jlpt_word_map: HashMap<u8, Vec<u32>>,
    // genki_levels: HashMap<u8, Vec<u32>>,
}

#[derive(Default)]
pub struct KanjiData {
    kanji: KanjiStorage,
    genki_levels: HashMap<u8, Vec<char>>,
    jlpt_data: HashMap<u8, Vec<char>>,
}

/// Contains sentences and jlpt levels for the dictionary.
#[derive(Serialize, Deserialize, Default)]
pub struct SentenceStorage {
    pub sentences: IntMap<Sentence>,
    pub jlpt_map: HashMap<u8, Vec<u32>>,
}

/// Contains all data for the dictionary suggestions.
#[derive(Default)]
pub(crate) struct SuggestionData {
    foregin: HashMap<Language, SuggestionDictionary<ForeignSuggestion>>,
    japanese: Option<SuggestionDictionary<NativeSuggestion>>,
}

impl DictionaryData {
    #[inline]
    fn new(
        words: WordData,
        names: NameStorage,
        kanji: KanjiData,
        rad_map: RadicalStorage,
        sentences: SentenceStorage,
        radicals: HashMap<char, DetailedRadical>,
    ) -> Self {
        Self {
            word_data: words,
            names,
            kanji,
            rad_map,
            sentences,
            radicals,
        }
    }

    /// Sets the word storage
    pub fn set_words(&mut self, words: Vec<Word>) {
        self.word_data.words = build_words(words);
    }
}

impl SuggestionData {
    #[inline]
    pub(super) fn new() -> Self {
        Self::default()
    }

    /// Returns false if there aren't suggestionts at all
    #[inline]
    pub(super) fn is_empty(&self) -> bool {
        self.japanese.is_none() && self.foregin.is_empty()
    }

    pub(super) fn add_foreign(
        &mut self,
        lang: Language,
        dict: SuggestionDictionary<ForeignSuggestion>,
    ) {
        self.foregin.insert(lang, dict);
    }

    pub(super) fn add_jp(&mut self, dict: SuggestionDictionary<NativeSuggestion>) {
        self.japanese = Some(dict);
    }
}

impl ResourceStorage {
    /// Create a new `ResourceStorage` by `Resources`
    #[inline]
    pub(super) fn new(
        resources: DictResources,
        suggestions: Option<SuggestionData>,
        rad_map: RadicalStorage,
        sentences: SentenceStorage,
    ) -> Self {
        let words = build_words(resources.words);
        let names = build_names(resources.names);
        let kanji = build_kanji(resources.kanji);
        let radicals = build_radicals(resources.radicals);

        let word_data = WordData {
            words,
            jlpt_word_map: resources.word_jlpt,
            // genki_levels: HashMap::new(),
        };

        let kanji_data = KanjiData {
            kanji,
            genki_levels: resources.kanji_genki,
            jlpt_data: resources.kanji_jlpt,
        };

        let dict_data =
            DictionaryData::new(word_data, names, kanji_data, rad_map, sentences, radicals);

        Self {
            dict_data,
            suggestions,
        }
    }

    /// Returns a `WordRetrieve` which can be used to retrieve words from the `ResourceStorage`
    #[inline]
    pub fn words<'a>(&'a self) -> WordRetrieve<'a> {
        WordRetrieve::new(self)
    }

    /// Returns an iterator over all words with given `jlpt` level
    pub fn word_jlpt(&self, jlpt: u8) -> Option<impl Iterator<Item = &'_ Word>> {
        let word = self.words();

        Some(
            self.dict_data
                .word_data
                .jlpt_word_map
                .get(&jlpt)?
                .iter()
                .filter_map(move |i| word.by_sequence(*i)),
        )
    }

    /// Returns an iterator over all sentences with given `jlpt` level
    pub fn sentence_jlpt(&self, jlpt: u8) -> Option<impl Iterator<Item = &'_ Sentence>> {
        let sentences = self.sentences();

        Some(
            self.dict_data
                .sentences
                .jlpt_map
                .get(&jlpt)?
                .iter()
                .filter_map(move |i| sentences.by_id(*i)),
        )
    }

    /// Returns the amount of sentences with given jlpt level
    pub fn sentence_jlpt_len(&self, jlpt: u8) -> usize {
        self.dict_data
            .sentences
            .jlpt_map
            .get(&jlpt)
            .map(|i| i.len())
            .unwrap_or_default()
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

    /// Returns a `SentenceRetrieve` which can be used to retrieve sentences from the `ResourceStorage`
    #[inline]
    pub fn sentences(&self) -> SentenceRetrieve<'_> {
        SentenceRetrieve::new(self)
    }
}

#[inline]
pub fn build_words(words: Vec<Word>) -> WordStorage {
    words.into_iter().map(|i| (i.sequence as u64, i)).collect()
}

#[inline]
fn build_names(names: Vec<Name>) -> NameStorage {
    names.into_iter().map(|i| (i.sequence as u64, i)).collect()
}

#[inline]
fn build_kanji(kanji: Vec<Kanji>) -> KanjiStorage {
    kanji.into_iter().map(|i| (i.literal, i)).collect()
}

#[inline]
fn build_radicals(radicals: Vec<DetailedRadical>) -> HashMap<char, DetailedRadical> {
    radicals.into_iter().map(|i| (i.literal, i)).collect()
}
