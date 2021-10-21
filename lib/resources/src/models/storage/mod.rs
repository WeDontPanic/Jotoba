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
    kanji::Kanji,
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

#[derive(Serialize, Deserialize, Default)]
pub struct SentenceStorage {
    pub sentences: IntMap<Sentence>,
    pub jlpt_map: HashMap<u8, Vec<u32>>,
}

#[derive(Default)]
pub struct ResourceStorage {
    pub dict_data: DictionaryData,
    suggestions: Option<SuggestionData>,
}

#[derive(Default)]
pub struct DictionaryData {
    words: WordStorage,
    jlpt_word_map: HashMap<u8, Vec<u32>>,
    names: NameStorage,
    kanji: KanjiStorage,
    rad_map: RadicalStorage,
    sentences: SentenceStorage,
}

#[derive(Default)]
pub(crate) struct SuggestionData {
    foregin: HashMap<Language, SuggestionDictionary<ForeignSuggestion>>,
    japanese: Option<SuggestionDictionary<NativeSuggestion>>,
}

impl DictionaryData {
    #[inline]
    fn new(
        words: WordStorage,
        jlpt_word_map: HashMap<u8, Vec<u32>>,
        names: NameStorage,
        kanji: KanjiStorage,
        rad_map: RadicalStorage,
        sentences: SentenceStorage,
    ) -> Self {
        Self {
            words,
            jlpt_word_map,
            names,
            kanji,
            rad_map,
            sentences,
        }
    }

    /// Sets the word storage
    pub fn set_words(&mut self, words: Vec<Word>) {
        self.words = build_words(words);
    }
}

impl SuggestionData {
    #[inline]
    pub(super) fn new() -> Self {
        Self::default()
    }

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

        let dict_data =
            DictionaryData::new(words, resources.word_jlpt, names, kanji, rad_map, sentences);

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
    pub fn word_jlpt<'a>(&'a self, jlpt: u8) -> Option<impl Iterator<Item = &'a Word>> {
        let word = self.words();

        Some(
            self.dict_data
                .jlpt_word_map
                .get(&jlpt)?
                .iter()
                .filter_map(move |i| word.by_sequence(*i)),
        )
    }

    /// Returns an iterator over all sentences with given `jlpt` level
    pub fn sentence_jlpt<'a>(&'a self, jlpt: u8) -> Option<impl Iterator<Item = &'a Sentence>> {
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
    pub fn names<'a>(&'a self) -> NameRetrieve<'a> {
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
    pub fn sentences<'a>(&'a self) -> SentenceRetrieve<'a> {
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
