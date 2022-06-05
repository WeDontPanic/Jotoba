pub mod dialect;
pub mod dict;
pub mod field;
pub mod foreign_language;
pub mod gtype;
pub mod inflection;
pub mod information;
pub mod misc;
pub mod part_of_speech;
pub mod pitch;
pub mod priority;
pub mod sense;

pub use dict::Dict;

use self::{
    inflection::Inflections,
    misc::Misc,
    part_of_speech::{PartOfSpeech, PosSimple},
    pitch::{raw_data::PitchValues, Pitch},
    priority::Priority,
    sense::{Sense, SenseGlossIter},
};
use super::languages::Language;
use bitflags::BitFlag;
use itertools::Itertools;
use serde::{Deserialize, Serialize};

#[cfg(feature = "jotoba_intern")]
use japanese::{
    furigana::{self, SentencePartRef},
    JapaneseExt,
};

/// A single word item
#[derive(Clone, Default, Serialize, Deserialize, Eq)]
pub struct Word {
    pub sequence: u32,
    pub priorities: Option<Vec<Priority>>,
    pub reading: Reading,
    pub senses: Vec<Sense>,
    pub furigana: Option<String>,
    pub jlpt_lvl: Option<u8>,
    pub collocations: Option<Vec<u32>>,
    pub transive_verion: Option<u32>,
    pub intransive_verion: Option<u32>,
    pub sentences_available: u16,
    pub accents: PitchValues,
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
        //self.reading.get_reading().priorities.is_some()
        self.reading_iter(true).any(|i| i.priorities.is_some())
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

    /// Returns an iterator over all sense and its glosses
    #[inline]
    pub fn sense_gloss_iter(&self) -> SenseGlossIter {
        SenseGlossIter::new(&self)
    }

    /// Return all senses of a language
    #[inline]
    pub fn senses_by_lang(&self, language: Language) -> Vec<&Sense> {
        self.senses
            .iter()
            .filter(|i| i.language == language)
            .collect()
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

    #[inline]
    pub fn sense_by_id(&self, id: u8) -> Option<&Sense> {
        self.senses.get(id as usize)
    }

    #[inline]
    pub fn get_sense_gloss(&self, id: u16) -> Option<(&Sense, &sense::Gloss)> {
        let (sense_id, gloss_id) = sense::from_unique_id(id);
        let sense = self.sense_by_id(sense_id)?;
        let gloss = sense.gloss_by_id(gloss_id)?;
        Some((sense, gloss))
    }

    /// Get amount of tags which will be displayed below the reading
    #[inline]
    pub fn get_word_tag_count(&self) -> u8 {
        [self.is_common(), self.get_jlpt_lvl().is_some()]
            .iter()
            .filter(|b| **b)
            .count() as u8
    }

    /// Returns an [`Inflections`] value if [`self`] is a valid verb
    #[inline]
    pub fn get_inflections(&self) -> Option<Inflections> {
        inflection::of_word(self)
    }

    /// Returns `true` if the word has at least one sentence in the given language
    #[inline]
    pub fn has_sentence(&self, language: Language) -> bool {
        let lang: i32 = language.into();
        BitFlag::<u16>::from(self.sentences_available).get(lang as u16)
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

    /// Returns an iterator over all reading elements
    #[inline]
    pub fn reading_iter(&self, allow_kana: bool) -> ReadingIter<'_> {
        self.reading.iter(allow_kana)
    }

    /// Returns true if word has `reading`
    pub fn has_reading(&self, reading: &str) -> bool {
        self.reading_iter(true).any(|j| j.reading == reading)
    }

    /// Returns `true` if `word` has `reading` as main (main kanji or kana reading)
    #[inline]
    pub fn has_main_reading(&self, reading: &str) -> bool {
        self.reading.kana.reading == reading
            || self
                .reading
                .kanji
                .as_ref()
                .map(|i| i.reading == reading)
                .unwrap_or(false)
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

// Jotoba intern only features
#[cfg(feature = "jotoba_intern")]
impl Word {
    /// Return `true` if the word is a katakana word
    #[inline]
    pub fn is_katakana_word(&self) -> bool {
        self.reading.is_katakana()
    }

    /// Get the audio path of a word
    #[inline]
    pub fn audio_file(&self, file_ending: &str) -> Option<String> {
        self.reading.kanji.as_ref().and_then(|kanji| {
            let file = format!(
                "{}/{}【{}】.{}",
                file_ending, kanji.reading, self.reading.kana.reading, file_ending
            );
            std::path::Path::new(&format!("html/audio/{}", file))
                .exists()
                .then(|| file)
        })
    }

    #[inline]
    pub fn get_kana(&self) -> &str {
        &self.reading.kana.reading
    }

    /// Returns a renderable vec of accents with kana characters
    #[inline]
    pub fn get_pitches(&self) -> Vec<Pitch> {
        self.accents
            .iter()
            .filter_map(|drop| Pitch::new(self.get_kana(), drop))
            .collect()
    }

    /// Returns a renderable vec of accents with kana characters
    #[inline]
    pub fn get_first_pitch(&self) -> Option<Pitch> {
        let drop = self.accents.get(0)?;
        Pitch::new(self.get_kana(), drop)
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

    fn pretty_print_senses(senses: &[Sense]) -> String {
        senses
            .iter()
            .map(|i| i.glosses.clone())
            .flatten()
            .into_iter()
            .map(|i| i.gloss)
            .join(", ")
    }
}

// Jotoba intern only features
#[cfg(feature = "jotoba_intern")]
impl Reading {
    /// Return `true` if reading represents a katakana only word
    #[inline]
    pub fn is_katakana(&self) -> bool {
        self.kana.reading.is_katakana() && self.kanji.is_none()
    }
}

impl Reading {
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
            return Some(self.reading.kanji.as_ref().unwrap());
        }
        let i = self.reading.alternative.get(self.alternative_pos)?;
        self.alternative_pos += 1;
        Some(i)
    }
}

#[cfg(feature = "jotoba_intern")]
#[inline]
pub fn adjust_language(word: &mut Word, language: Language, show_english: bool) {
    word.senses
        .retain(|j| j.language == language || (j.language == Language::English && show_english));
}

/// Removes all senses which ain't in the provided language or english in case `show_english` is
/// `true`
#[cfg(feature = "jotoba_intern")]
pub fn filter_languages<'a, I: 'a + Iterator<Item = &'a mut Word>>(
    iter: I,
    language: Language,
    show_english: bool,
) {
    for word in iter {
        adjust_language(word, language, show_english);
    }
}

impl std::hash::Hash for Word {
    #[inline]
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.sequence.hash(state);
    }
}

impl PartialEq for Word {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.sequence == other.sequence
    }
}

impl std::fmt::Debug for Word {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let senses = self
            .senses_by_lang(Language::English)
            .into_iter()
            .map(|i| i.glosses.iter().map(|i| &i.gloss).join("|"))
            .join("\n");

        f.debug_struct("Word")
            .field("Seq", &self.sequence)
            .field("Kana", &self.reading.kana.reading)
            .field("Reading", &self.get_reading().reading)
            .field("Common", &self.is_common())
            .field("JLPT", &self.jlpt_lvl)
            .field("Translations", &senses)
            .finish()
    }
}
