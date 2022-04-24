pub mod kanji;
pub mod name;
pub mod sentence;
pub mod word;

use intmap::IntMap;

use serde::{Deserialize, Serialize};

use self::{
    kanji::KanjiRetrieve, name::NameRetrieve, sentence::SentenceRetrieve, word::WordRetrieve,
};
use super::DictResources;
use std::collections::HashMap;
use types::jotoba::{
    kanji::{DetailedRadical, Kanji},
    names::Name,
    sentences::Sentence,
    words::Word,
};

pub type WordStorage = IntMap<Word>;
pub type NameStorage = IntMap<Name>;
pub type KanjiStorage = HashMap<char, Kanji>;
/// Maps radicals to all kanji using the raical
pub type RadicalStorage = HashMap<char, Vec<char>>;

/// A dictionary of words, names, kanji, and radicals. This is the main data structure for the dictionary.
#[derive(Default)]
pub struct ResourceStorage {
    pub dict_data: DictionaryData,
}

/// Contains all core data for the dictionary. This is the data structure for the dictionary functionality to work properly.
#[derive(Default)]
pub struct DictionaryData {
    word_data: WordData,
    names: NameStorage,
    kanji: KanjiData,
    rad_kanji_map: RadicalStorage,
    sentences: SentenceStorage,
    radicals: HashMap<char, DetailedRadical>,
}

/// Represents the dictionary word data
#[derive(Default)]
pub struct WordData {
    words: WordStorage,
    jlpt_word_map: HashMap<u8, Vec<u32>>,
    irregular_ichidan: Vec<u32>,
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

impl DictionaryData {
    #[inline]
    fn new(
        word_data: WordData,
        names: NameStorage,
        kanji: KanjiData,
        rad_kanji_map: RadicalStorage,
        sentences: SentenceStorage,
        radicals: HashMap<char, DetailedRadical>,
    ) -> Self {
        Self {
            word_data,
            names,
            kanji,
            rad_kanji_map,
            sentences,
            radicals,
        }
    }

    /// Sets the word storage
    pub fn set_words(&mut self, words: Vec<Word>) {
        self.word_data.words = build_words(words);
    }
}

impl ResourceStorage {
    /// Create a new `ResourceStorage` by `Resources`
    #[inline]
    pub(super) fn new(
        resources: DictResources,
        rad_kanji_map: RadicalStorage,
        sentences: SentenceStorage,
    ) -> Self {
        let words = build_words(resources.words);
        let names = build_names(resources.names);
        let kanji = build_kanji(resources.kanji);
        let radicals = build_radicals(resources.radicals);

        let word_data = WordData {
            words,
            jlpt_word_map: resources.word_jlpt,
            irregular_ichidan: resources.irregular_iru_eru,
        };

        let kanji_data = KanjiData {
            kanji,
            genki_levels: resources.kanji_genki,
            jlpt_data: resources.kanji_jlpt,
        };

        let dict_data = DictionaryData::new(
            word_data,
            names,
            kanji_data,
            rad_kanji_map,
            sentences,
            radicals,
        );

        Self { dict_data }
    }

    /// Returns a `WordRetrieve` which can be used to retrieve words from the `ResourceStorage`
    #[inline]
    pub fn words<'a>(&'a self) -> WordRetrieve<'a> {
        WordRetrieve::new(self)
    }

    /// Returns an iterator over all words with given `jlpt` level
    pub fn word_jlpt(&self, jlpt: u8) -> impl Iterator<Item = &'_ Word> {
        let word = self.words();

        self.dict_data
            .word_data
            .jlpt_word_map
            .get(&jlpt)
            .into_iter()
            .flatten()
            .filter_map(move |i| word.by_sequence(*i))
    }

    /// Returns an iterator over all sentences with given `jlpt` level
    pub fn sentence_jlpt(&self, jlpt: u8) -> impl Iterator<Item = &'_ Sentence> {
        let sentences = self.sentences();

        self.dict_data
            .sentences
            .jlpt_map
            .get(&jlpt)
            .into_iter()
            .flatten()
            .filter_map(move |i| sentences.by_id(*i))
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
    pub fn names(&'static self) -> NameRetrieve<'static> {
        NameRetrieve::new(self)
    }

    /// Returns a `KanjiRetrieve` which can be used to retrieve kanji from the `ResourceStorage`
    #[inline]
    pub fn kanji(&self) -> KanjiRetrieve<'_> {
        KanjiRetrieve::new(self)
    }

    /// Returns a `SentenceRetrieve` which can be used to retrieve sentences from the `ResourceStorage`
    #[inline]
    pub fn sentences(&self) -> SentenceRetrieve<'_> {
        SentenceRetrieve::new(self)
    }
}

pub fn build_words(words: Vec<Word>) -> WordStorage {
    words.into_iter().map(|i| (i.sequence as u64, i)).collect()
}

pub fn build_names(names: Vec<Name>) -> NameStorage {
    names.into_iter().map(|i| (i.sequence as u64, i)).collect()
}

pub fn build_kanji(kanji: Vec<Kanji>) -> KanjiStorage {
    kanji.into_iter().map(|i| (i.literal, i)).collect()
}

pub fn build_radicals(radicals: Vec<DetailedRadical>) -> HashMap<char, DetailedRadical> {
    radicals.into_iter().map(|i| (i.literal, i)).collect()
}
