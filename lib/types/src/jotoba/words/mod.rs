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
pub mod reading;
pub mod sense;

pub use dict::Dict;

use super::language::{param::AsLangParam, Language};
use bitflags::BitFlag;
use itertools::Itertools;
use jp_utils::{
    furigana::{self, reading_part_ref::ReadingPartRef},
    JapaneseExt,
};
use misc::Misc;
use part_of_speech::{PartOfSpeech, PosSimple};
use pitch::{raw_data::PitchValues, Pitch};
use reading::{Reading, ReadingIter};
use sense::{Sense, SenseGlossIter};
use serde::{Deserialize, Serialize};
use std::{
    hash::{Hash, Hasher},
    num::{NonZeroU32, NonZeroU8},
    path::Path,
};

/// A single word in Jotobas word search
#[derive(Clone, Default, Serialize, Deserialize, Eq)]
pub struct Word {
    pub sequence: u32,
    pub common: bool,
    pub reading: Reading,
    pub senses: Vec<Sense>,
    pub furigana: Option<String>,
    pub jlpt_lvl: Option<NonZeroU8>,
    pub collocations: Option<Vec<u32>>,
    pub transive_version: Option<NonZeroU32>,
    pub intransive_version: Option<NonZeroU32>,
    pub sentences_available: u16,
    pub accents: PitchValues,
}

impl Word {
    /// Returns true if a word is common
    #[inline]
    pub fn is_common(&self) -> bool {
        self.common
    }

    /// Returns the jlpt level of a word. `None` if a word doesn't have a JLPT lvl assigned
    #[inline]
    pub fn get_jlpt_lvl(&self) -> Option<u8> {
        self.jlpt_lvl.map(|i| i.get())
    }

    /// Returns the main reading of a word. This is the kanji reading if a kanji reading
    /// exists. Otherwise its the kana reading
    #[inline]
    pub fn get_reading(&self) -> &Dict {
        self.reading.get_reading()
    }

    /// Returns the main reading of a word as str. This is the kanji reading if a kanji reading
    /// exists. Otherwise its the kana reading
    #[inline]
    pub fn get_reading_str(&self) -> &str {
        &self.get_reading().reading
    }

    /// Returns an iterator over all sense and its glosses
    #[inline]
    pub fn sense_gloss_iter(&self) -> SenseGlossIter {
        SenseGlossIter::new(&self)
    }

    /// Return all senses of a language
    #[inline]
    pub fn senses_by_lang(&self, language: impl AsLangParam) -> Vec<&Sense> {
        let language = language.as_lang();
        self.senses
            .iter()
            .filter(|i| language.eq_to_lang(&i.language))
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
    pub fn get_senses_with_en(&self) -> Vec<Vec<Sense>> {
        let (english, other): (Vec<Sense>, Vec<Sense>) = self
            .senses
            .clone()
            .into_iter()
            .partition(|i| i.language == Language::English);

        vec![other, english]
    }

    /// Returns all senses of the word
    #[inline]
    pub fn senses(&self) -> &[Sense] {
        &self.senses
    }

    #[inline]
    pub fn sense_by_id(&self, id: u8) -> Option<&Sense> {
        self.senses.get(id as usize)
    }

    pub fn get_sense_gloss(&self, id: u16) -> Option<(&Sense, &sense::Gloss)> {
        let (sense_id, gloss_id) = sense::from_unique_id(id);
        let sense = self.sense_by_id(sense_id)?;
        let gloss = sense.gloss_by_id(gloss_id)?;
        Some((sense, gloss))
    }

    /// Returns an Iterator over the words glosses using a given language
    pub fn gloss_iter_by_lang(&self, lang_param: impl AsLangParam) -> impl Iterator<Item = &str> {
        let lang_param = lang_param.as_lang();
        self.sense_gloss_iter()
            .filter(move |i| lang_param.eq_to_lang(&i.0.language))
            .map(|i| i.1.gloss.as_str())
    }

    /// Get amount of tags which will be displayed below the reading
    pub fn get_word_tag_count(&self) -> u8 {
        [self.is_common(), self.get_jlpt_lvl().is_some()]
            .iter()
            .filter(|b| **b)
            .count() as u8
    }

    /// Returns `true` if the word has at least one sentence in the given language
    pub fn has_sentence(&self, lang: impl AsLangParam) -> bool {
        let lang_p = lang.as_lang();
        let lang: i32 = lang_p.language().into();

        BitFlag::<u16>::from(self.sentences_available).get(lang as u16)
            || (lang_p.en_fallback()
                && !lang_p.is_english()
                && BitFlag::<u16>::from(self.sentences_available).get(Language::English as u16))
    }

    /// Returns true if word has a misc information matching `misc`. This requires english glosses
    /// to be available since they're the only one holding misc information
    #[inline]
    pub fn has_misc(&self, misc: &Misc) -> bool {
        self.senses
            .iter()
            .filter_map(|i| i.misc)
            .any(|i| i == *misc)
    }

    /// Returns `true` if word has at least one of the provided part of speech
    pub fn has_pos(&self, pos_filter: &[PosSimple]) -> bool {
        for sense in self.senses.iter().map(|i| i.get_pos_simple()) {
            if sense.iter().any(|i| pos_filter.contains(i)) {
                return true;
            }
        }

        false
    }

    /// Returns `true` if word has all of the provided part of speech
    #[inline]
    pub fn has_all_pos(&self, pos_filter: &[PosSimple]) -> bool {
        self.has_all_pos_iter(pos_filter.iter())
    }

    /// Returns `true` if word has all of the provided part of speech
    #[inline]
    pub fn has_all_pos_iter<'a, I>(&self, mut pos_filter: I) -> bool
    where
        I: Iterator<Item = &'a PosSimple> + 'a,
    {
        pos_filter.all(|pos| self.senses.iter().any(|s| s.has_pos_simple(pos)))
    }

    /// Returns `true` if a word has at least one translation for the provided language, or english
    /// if `allow_english` is `true`
    #[inline]
    pub fn has_language(&self, language: impl AsLangParam) -> bool {
        let lang = language.as_lang();
        self.senses.iter().any(|i| lang.eq_to_lang(&i.language))
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
    #[inline]
    pub fn has_reading(&self, reading: &str) -> bool {
        self.reading_iter(true).any(|j| j.reading == reading)
    }

    /// Returns `true` if the word has a kanji reading
    #[inline]
    pub fn has_kanji(&self) -> bool {
        self.get_reading_str().has_kanji()
    }

    /// Returns `true` if `word` has `reading` as main (main kanji or kana reading)
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
    pub fn get_pos(&self) -> impl Iterator<Item = &PartOfSpeech> {
        self.senses
            .iter()
            .map(|i| i.part_of_speech.iter())
            .flatten()
    }

    #[inline]
    pub fn get_kana(&self) -> &str {
        &self.reading.kana.reading
    }

    #[inline]
    pub fn has_pitch(&self) -> bool {
        !self.accents.is_empty()
    }

    /// Returns a renderable vec of accents with kana characters
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

    /// Return `true` if the word is a katakana word
    #[inline]
    pub fn is_katakana_word(&self) -> bool {
        self.reading.is_katakana()
    }

    /// Removes all languages except the one specified and potentionally english when enabled
    #[inline]
    pub fn adjust_language(&mut self, lang: impl AsLangParam) {
        let lang = lang.as_lang();
        self.senses.retain(|j| lang.eq_to_lang(&j.language));
    }

    /// Returns furigana reading-pairs of an Item
    #[inline]
    pub fn get_furigana(&self) -> Option<Vec<ReadingPartRef<'_>>> {
        let furi = self.furigana.as_ref()?;
        furigana::parse::full(&furi).ok()
    }
}

// Jotoba intern only features
#[cfg(feature = "jotoba_intern")]
impl Word {
    /// Get the audio's filename of the word
    #[inline]
    pub fn audio_file_name(&self) -> Option<String> {
        self.reading
            .kanji
            .as_ref()
            .map(|kanji| format!("{}【{}】.mp3", kanji.reading, self.reading.kana.reading))
    }

    /// Get the audio's filename of the word
    #[inline]
    pub fn audio_file_name_old(&self) -> Option<String> {
        self.reading.kanji.as_ref().and_then(|kanji| {
            /* let frame_path = format!("svg/kanji/{}_frames.svg", self.literal);
            let frame_path = Path::new(&frame_path);
            assets_path.as_ref().join(frame_path) */

            let file = format!("{}【{}】.mp3", kanji.reading, self.reading.kana.reading);
            std::path::Path::new(&format!("html/audio/mp3/{}", file))
                .exists()
                .then(|| file)
        })
    }

    /// Get the audio path of a word
    #[inline]
    pub fn audio_file<P: AsRef<Path>>(&self, assets_path: P) -> Option<String> {
        self.reading.kanji.as_ref().and_then(|kanji| {
            let file = format!("mp3/{}【{}】.mp3", kanji.reading, self.reading.kana.reading);
            std::path::Path::new(&format!("html/audio/{}", file))
                .exists()
                .then(|| file)
        })
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
        let senses = self.get_senses_with_en();

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

    /// Returns an [`Inflections`] value if [`self`] is a valid verb
    #[inline]
    pub fn get_inflections(&self) -> Option<inflection::Inflections> {
        inflection::of_word(self)
    }
}

/// Removes all senses which ain't in the provided language or english in case `show_english` is
/// `true`
#[cfg(feature = "jotoba_intern")]
pub fn filter_languages<'a, I: 'a + Iterator<Item = &'a mut Word>>(
    iter: I,
    lang: impl AsLangParam,
) {
    for word in iter {
        word.adjust_language(lang);
    }
}

impl Hash for Word {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
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
