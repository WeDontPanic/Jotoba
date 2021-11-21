pub mod dict;
pub mod inflection;
pub mod sense;

use bitflags::BitFlag;
pub use dict::Dict;
use itertools::Itertools;
pub use sense::{Gloss, Sense};
use utils::to_option;

use crate::parse::jmdict::{
    languages::Language,
    misc::Misc,
    part_of_speech::{PartOfSpeech, PosSimple},
    priority::Priority,
};
use japanese::{
    accent::{AccentChar, Border},
    furigana::{self, SentencePartRef},
    JapaneseExt,
};
use serde::{Deserialize, Serialize};
use std::path::Path;

use self::inflection::Inflections;

/// A single word item
#[derive(Debug, Clone, Default, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub struct Word {
    pub sequence: u32,
    pub priorities: Option<Vec<Priority>>,
    pub reading: Reading,
    pub senses: Vec<Sense>,
    pub accents: Option<Vec<u8>>,
    pub furigana: Option<String>,
    pub jlpt_lvl: Option<u8>,
    pub collocations: Option<Vec<u32>>,
    pub transive_verion: Option<u32>,
    pub intransive_verion: Option<u32>,
    pub sentences_available: u16,
}

/// Various readings of a word
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize, Hash, Eq)]
pub struct Reading {
    pub kana: Dict,
    pub kanji: Option<Dict>,
    pub alternative: Vec<Dict>,
}

impl Word {
    /// Returns true if a word is common
    #[inline]
    pub fn is_common(&self) -> bool {
        self.reading.get_reading().priorities.is_some()
    }

    /// Returns the jlpt level of a word. `None` if a word doesn't have a JLPT lvl assigned
    #[inline]
    pub fn get_jlpt_lvl(&self) -> Option<u8> {
        self.jlpt_lvl
    }

    /// Returns the reading of a word
    #[inline]
    pub fn get_reading(&self) -> &Dict {
        self.reading.get_reading()
    }

    /// Return `true` if the word is a katakana word
    #[inline]
    pub fn is_katakana_word(&self) -> bool {
        self.reading.is_katakana()
    }

    /// Return all senses of a language
    #[inline]
    pub fn senses_by_lang(&self, language: Language) -> Option<Vec<Sense>> {
        let senses = self
            .senses
            .iter()
            .filter(|i| i.language == language)
            .cloned()
            .collect();

        to_option(senses)
    }

    /// Get senses ordered by language (non-english first)
    pub fn get_senses_orderd(&self, english_on_top: bool, _language: Language) -> Vec<Vec<Sense>> {
        let (english, other): (Vec<Sense>, Vec<Sense>) = self
            .senses
            .clone()
            .into_iter()
            .partition(|i| i.language == Language::English);

        if english_on_top {
            vec![english, other]
        } else {
            vec![other, english]
        }
    }

    /// Get senses ordered by language (non-english first)
    pub fn get_senses(&self) -> Vec<Vec<Sense>> {
        let (english, other): (Vec<Sense>, Vec<Sense>) = self
            .senses
            .clone()
            .into_iter()
            .partition(|i| i.language == Language::English);

        vec![other, english]
    }

    /// Get amount of tags which will be displayed below the reading
    #[inline]
    pub fn get_word_tag_count(&self) -> u8 {
        [self.is_common(), self.get_jlpt_lvl().is_some()]
            .iter()
            .filter(|b| **b)
            .count() as u8
    }

    /// Get the audio path of a word
    #[inline]
    pub fn audio_file(&self, file_ending: &str) -> Option<String> {
        self.reading.kanji.as_ref().and_then(|kanji| {
            let file = format!("{}/{}【{}】.{}", file_ending, kanji.reading, self.reading.kana.reading, file_ending);
            Path::new(&format!("html/audio/{}", file))
                .exists()
                .then(|| file)
        })
    }

    /// Returns a renderable vec of accents with kana characters
    pub fn get_accents(&self) -> Option<Vec<AccentChar>> {
        let accents_raw = self.accents.as_ref()?;
        let kana = &self.reading.kana;
        let accents = japanese::accent::calc_pitch(&kana.reading, accents_raw[0] as i32)?;
        let accent_iter = accents.iter().peekable().enumerate();

        let res = accent_iter
            .map(|(pos, (part, is_high))| {
                if part.len() == 0 {
                    // Don't render under/overline for empty character -- handles the case where the 
                    // pitch changes from the end of the word to the particle
                    return vec![];
                }
                let borders = vec![if *is_high {
                    Border::Top
                } else {
                    Border::Bottom
                }];
                let borders = if pos != accents.len() - 1 {
                    borders.into_iter().chain(vec![Border::Right]).collect()
                } else {
                    borders
                };
                vec![AccentChar { borders, c: part }]
            })
            .flatten()
            .into_iter()
            .collect();

        Some(res)
    }

    /// Returns furigana reading-pairs of an Item
    #[inline]
    pub fn get_furigana(&self) -> Option<Vec<SentencePartRef<'_>>> {
        let furi = self.furigana.as_ref()?;
        Some(furigana::from_str(furi).collect::<Vec<_>>())
    }

    /// Get alternative readings in a beautified, print-ready format
    #[inline]
    pub fn alt_readings_beautified(&self) -> String {
        self.reading
            .alternative
            .iter()
            .map(|i| i.reading.clone())
            .join(", ")
    }

    /// Returns an [`Inflections`] value if [`self`] is a valid verb
    #[inline]
    pub fn get_inflections(&self) -> Option<Inflections> {
        inflection::of_word(self)
    }

    #[inline]
    pub fn get_transitive_counterpart(&self) -> Option<Word> {
        let seq_id = self.transive_verion.as_ref()?;
        crate::get().words().by_sequence(*seq_id).cloned()
    }

    #[inline]
    pub fn get_intransitive_counterpart(&self) -> Option<Word> {
        let seq_id = self.intransive_verion.as_ref()?;
        crate::get().words().by_sequence(*seq_id).cloned()
    }

    /// Returns `true` if the word has at least one sentence in the given language
    #[inline]
    pub fn has_sentence(&self, language: Language) -> bool {
        let lang: i32 = language.into();
        BitFlag::<u16>::from(self.sentences_available).get(lang as u16)
    }

    pub fn glosses_pretty(&self) -> String {
        let senses = self.get_senses();

        // Try to use glosses with users language
        if !senses[0].is_empty() {
            Self::pretty_print_senses(&senses[0])
        } else {
            // Fallback use english gloses
            Self::pretty_print_senses(&senses[1])
        }
    }

    /// Returns true if word has a misc information matching `misc`. This requires english glosses
    /// to be available since they're the only one holding misc information
    #[inline]
    pub fn has_misc(&self, misc: Misc) -> bool {
        self.senses.iter().filter_map(|i| i.misc).any(|i| i == misc)
    }

    /// Returns `true` if word has at least one of the provided part of speech
    #[inline]
    pub fn has_pos(&self, pos_filter: &[PosSimple]) -> bool {
        for sense in self.senses.iter().map(|i| i.get_pos_simple()) {
            if sense.iter().any(|i| pos_filter.contains(i)) {
                return true;
            }
        }

        false
    }

    /// Returns `true` if a word has at least one translation for the provided language, or english
    /// if `allow_english` is `true`
    #[inline]
    pub fn has_language(&self, language: Language, allow_english: bool) -> bool {
        self.senses
            .iter()
            .any(|i| i.language == language || (allow_english && i.language == Language::English))
    }

    /// Returns `true` if a word has collocations
    #[inline]
    pub fn has_collocations(&self) -> bool {
        self.collocations.is_some()
    }

    /// Returns a list of all collocations of a word
    pub fn get_collocations(
        &self,
        language: Language,
        show_english: bool,
    ) -> Vec<(String, String)> {
        if !self.has_collocations() {
            return vec![];
        }

        let word_storage = crate::get().words();

        let mut words = self
            .collocations
            .as_ref()
            .unwrap()
            .iter()
            .filter_map(|i| word_storage.by_sequence(*i))
            .cloned()
            .collect::<Vec<_>>();

        filter_languages(words.iter_mut(), language, show_english);

        let words = words
            .into_iter()
            .map(|word| {
                let senses: Vec<String> = word
                    .get_senses()
                    .into_iter()
                    .flatten()
                    .take(5)
                    .map(|i| i.glosses)
                    .flatten()
                    .map(|i| i.gloss)
                    .collect();

                let reading = word.reading.kanji.unwrap_or(word.reading.kana).reading;

                (reading, senses.join(", "))
            })
            .collect();

        words
    }

    /// Returns an iterator over all reading elements
    #[inline]
    pub fn reading_iter(&self, allow_kana: bool) -> ReadingIter<'_> {
        self.reading.iter(allow_kana)
    }

    fn pretty_print_senses(senses: &[Sense]) -> String {
        senses
            .iter()
            .map(|i| i.glosses.clone())
            .flatten()
            .into_iter()
            .map(|i| i.gloss)
            .join(", ")
    }

    /// Returns an iterator over all parts of speech of a word
    #[inline]
    fn get_pos(&self) -> impl Iterator<Item = &PartOfSpeech> {
        self.senses
            .iter()
            .map(|i| i.part_of_speech.iter())
            .flatten()
    }
}

impl Reading {
    /// Return `true` if reading represents a katakana only word
    #[inline]
    pub fn is_katakana(&self) -> bool {
        self.kana.reading.is_katakana() && self.kanji.is_none()
    }

    /// Returns the preferred word-reading of a `Reading`
    #[inline]
    pub fn get_reading(&self) -> &Dict {
        self.kanji.as_ref().unwrap_or(&self.kana)
    }

    /// Returns an iterator over all reading elements
    #[inline]
    pub fn iter(&self, allow_kana: bool) -> ReadingIter<'_> {
        ReadingIter::new(self, allow_kana)
    }
}

pub struct ReadingIter<'a> {
    reading: &'a Reading,
    allow_kana: bool,
    did_kanji: bool,
    did_kana: bool,
    alternative_pos: usize,
}

impl<'a> ReadingIter<'a> {
    #[inline]
    fn new(reading: &'a Reading, allow_kana: bool) -> Self {
        Self {
            reading,
            allow_kana,
            did_kana: false,
            did_kanji: false,
            alternative_pos: 0,
        }
    }
}

impl<'a> Iterator for ReadingIter<'a> {
    type Item = &'a Dict;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.did_kana && self.allow_kana {
            self.did_kana = true;
            return Some(&self.reading.kana);
        }
        if !self.did_kanji && self.reading.kanji.is_some() {
            self.did_kanji = true;
            return Some(&self.reading.kanji.as_ref().unwrap());
        }
        let i = self.reading.alternative.get(self.alternative_pos)?;
        self.alternative_pos += 1;
        Some(i)
    }
}

/// Removes all senses which ain't in the provided language or english in case `show_english` is
/// `true`
pub fn filter_languages<'a, I: 'a + Iterator<Item = &'a mut Word>>(
    iter: I,
    language: Language,
    show_english: bool,
) {
    for word in iter {
        word.senses.retain(|j| {
            j.language == language || (j.language == Language::English && show_english)
        });
    }
}
