use std::path::Path;

use serde::{Deserialize, Serialize};

/// A Kanji representing structure containing all available information about a single kanji
/// character.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Kanji {
    pub literal: char,
    pub grade: Option<u8>,
    pub stroke_count: u8,
    pub frequency: Option<u16>,
    pub jlpt: Option<u8>,
    pub variant: Option<Vec<String>>,
    pub onyomi: Option<Vec<String>>,
    pub kunyomi: Option<Vec<String>>,
    pub chinese: Option<Vec<String>>,
    pub korean_r: Option<Vec<String>>,
    pub korean_h: Option<Vec<String>>,
    pub natori: Option<Vec<String>>,
    pub kun_dicts: Option<Vec<u32>>,
    pub on_dicts: Option<Vec<u32>>,
    pub similar_kanji: Option<Vec<char>>,
    pub meanings: Vec<String>,
    pub radical: DetailedRadical,
    pub parts: Option<Vec<char>>,
}

/// A single radical representing structure
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct DetailedRadical {
    pub id: u16,
    pub literal: char,
    pub alternative: Option<char>,
    pub stroke_count: u8,
    pub readings: Vec<String>,
    pub translations: Option<Vec<String>>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct SearchRadicalInfo {
    pub literal: char,
    pub frequency: u16,
    pub meanings: Vec<String>,
}

/// Represents a radical which gets used for kanji-searches
#[derive(Debug, Clone, PartialEq)]
pub struct SearchRadical {
    pub radical: char,
    pub stroke_count: i32,
}

/// ReadingType of a kanji's reading. `Kunyomi` represents japanese readings and `Onyomi`
/// represents original chinese readings.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ReadingType {
    Kunyomi,
    Onyomi,
}

/// A kanji-reading search item
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct Reading {
    /// The provided kanji literal
    pub literal: char,
    /// The provided kanji reading
    pub reading: String,
}

impl Reading {
    #[inline]
    pub fn new(literal: &str, reading: &str) -> Self {
        Reading {
            literal: literal.chars().next().unwrap(),
            reading: reading.to_string(),
        }
    }
}

impl Kanji {
    /// Returns the `ReadingType` of `reading` within readings of a kanji
    pub fn get_reading_type(&self, reading: &str) -> Option<ReadingType> {
        let in_on = self.in_on_reading(reading);
        let in_kun = self.in_kun_reading(reading);

        if in_on && !in_kun {
            return Some(ReadingType::Onyomi);
        } else if !in_on && in_kun {
            return Some(ReadingType::Kunyomi);
        }

        None
    }

    /// Returns `true` if the kanji has `reading` within the `kunyomi`
    #[inline]
    pub fn in_kun_reading(&self, reading: &str) -> bool {
        self.kunyomi
            .as_ref()
            .map(|i| i.iter().any(|i| i.as_str() == reading))
            .unwrap_or_default()
    }

    /// Returns `true` if the kanji has `reading` within the `onyomi`
    #[inline]
    pub fn in_on_reading(&self, reading: &str) -> bool {
        self.onyomi
            .as_ref()
            .map(|i| i.iter().any(|i| i.as_str() == reading))
            .unwrap_or_default()
    }

    #[inline]
    pub fn get_literal_reading(&self, reading: &str) -> Option<String> {
        Some(match self.get_reading_type(reading)? {
            ReadingType::Kunyomi => literal_kun_reading(reading),
            ReadingType::Onyomi => format_reading(reading),
        })
    }

    /// Returns true if kanji has a given reading
    #[inline]
    pub fn has_reading(&self, reading: &str) -> bool {
        self.in_on_reading(reading) || self.in_kun_reading(reading)
    }

    /// Returns `true` if the kanji has stroke frames
    #[inline]
    pub fn has_stroke_frames(&self) -> bool {
        Path::new(&self.get_animation_path()).exists()
    }

    /// Returns the local path to stroke-frames svg
    #[inline]
    pub fn get_stroke_frames_url(&self) -> String {
        format!("/assets/svg/kanji/{}_frames.svg", self.literal)
    }

    /// Returns `true` if the kanji has a stroke animation file
    #[inline]
    pub fn has_animation_file(&self) -> bool {
        Path::new(&self.get_animation_path()).exists()
    }

    /// Returns the local path of the kanjis stroke-animation
    #[inline]
    pub fn get_animation_path(&self) -> String {
        format!("html/assets/svg/kanji/{}_animated.svgs", self.literal)
    }

    /// Returns `true` if kanji has on or kun compounds (or both)
    #[inline]
    pub fn has_compounds(&self) -> bool {
        self.on_dicts.is_some() || self.kun_dicts.is_some()
    }
}

/// Formats a kun/on reading to a kana entry
#[inline]
pub fn format_reading(reading: &str) -> String {
    reading.replace('-', "").replace('.', "")
}

/// Returns the reading of a kanjis literal, given the kun reading
#[inline]
pub fn literal_kun_reading(kun: &str) -> String {
    kun.replace('-', "").split('.').next().unwrap().to_string()
}

/// Formats `literal` with `reading`, based on `ReadingType`
///
/// Example:
///
/// literal: 捗
/// reading: はかど.る
/// r_type: ReadingType::Kunyomi
/// returns: 捗る
pub fn format_reading_with_literal(literal: char, reading: &str, r_type: ReadingType) -> String {
    match r_type {
        ReadingType::Kunyomi => {
            let r = if reading.contains('.') {
                let right = reading.split('.').nth(1).unwrap_or_default();
                format!("{}{}", literal, right)
            } else {
                literal.to_string()
            };
            r.replace("-", "")
        }
        ReadingType::Onyomi => literal.to_string(),
    }
}
