pub mod radical;
pub mod reading;

use std::{char, path::Path};

use serde::{Deserialize, Serialize};

use self::{
    radical::DetailedRadical,
    reading::{Reading, ReadingType},
};

/// A Kanji representing structure containing all available information about a single kanji
/// character.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Kanji {
    pub literal: char,
    pub grade: Option<u8>,
    pub stroke_count: u8,
    pub frequency: Option<u16>,
    pub jlpt: Option<u8>,
    pub variant: Vec<String>,
    pub onyomi: Vec<String>,
    /// Japanese name readings
    pub nanori: Vec<String>,
    pub kunyomi: Vec<String>,
    pub chinese: Vec<String>,
    pub korean_r: Vec<String>,
    pub korean_h: Vec<String>,
    pub vietnamese: Vec<String>,
    pub kun_dicts: Vec<u32>,
    pub on_dicts: Vec<u32>,
    pub similar_kanji: Vec<char>,
    pub meanings: Vec<String>,
    pub radical: DetailedRadical,
    pub parts: Vec<char>,
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
        self.kunyomi.iter().any(|i| i.as_str() == reading)
    }

    /// Returns `true` if the kanji has `reading` within the `onyomi`
    #[inline]
    pub fn in_on_reading(&self, reading: &str) -> bool {
        self.onyomi.iter().any(|i| i.as_str() == reading)
    }

    /// Tries to find the given reading in the kanjis readings and returns a `Reading` value if
    /// found
    pub fn find_reading(&self, reading: &str) -> Option<Reading> {
        let on = self.onyomi.iter().find(|i| i == &reading);
        let kun = self.kunyomi.iter().find(|i| i == &reading);

        let r = on.or(kun)?;

        let rt = if on.is_some() {
            ReadingType::Onyomi
        } else {
            ReadingType::Kunyomi
        };

        Some(Reading::new(rt, self.literal, r.to_string()))
    }

    /// Returns an iteratort over all readings
    pub fn reading_iter(&self) -> impl Iterator<Item = (&String, u32)> {
        self.kunyomi
            .iter()
            .chain(self.onyomi.iter())
            .enumerate()
            .map(|i| (i.1, i.0 as u32))
    }

    pub fn reading_from_pos(&self, pos: usize) -> Option<Reading> {
        if pos < self.kunyomi.len() {
            let r = self.kunyomi.get(pos).unwrap();
            Some(Reading::new(
                ReadingType::Kunyomi,
                self.literal,
                r.to_string(),
            ))
        } else {
            let k_len = self.kunyomi.len();
            let r = self.onyomi.get(pos - k_len)?;
            Some(Reading::new(
                ReadingType::Onyomi,
                self.literal,
                r.to_string(),
            ))
        }
    }

    #[deprecated(note = "use find_reading instead")]
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

    /// Returns the url to stroke-frames svg
    #[inline]
    pub fn get_stroke_frames_url(&self) -> String {
        format!("/assets/svg/kanji/{}_frames.svg", self.literal)
    }

    /// Returns the local path of the stroke-frames
    #[inline]
    pub fn get_stroke_frames_path(&self) -> String {
        format!("html/assets/svg/kanji/{}_frames.svg", self.literal)
    }

    /// Returns `true` if the kanji has a stroke animation file
    #[inline]
    pub fn has_animation_file(&self) -> bool {
        Path::new(&self.get_animation_path()).exists()
    }

    /// Returns the local path of the kanjis stroke-animation
    #[inline]
    pub fn get_animation_path(&self) -> String {
        format!("html/assets/svg/kanji/{}.svg", self.literal)
    }

    /// Returns `true` if kanji has on or kun compounds (or both)
    #[inline]
    pub fn has_compounds(&self) -> bool {
        (!self.on_dicts.is_empty()) || (!self.kun_dicts.is_empty())
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

#[cfg(test)]
mod test {
    use super::*;

    fn reading_on1() -> Reading {
        Reading::new(ReadingType::Onyomi, '長', "ちょう".to_string())
    }

    fn reading_kun() -> Reading {
        Reading::new(ReadingType::Kunyomi, '長', "なが.い".to_string())
    }

    fn reading_kun2() -> Reading {
        Reading::new(ReadingType::Kunyomi, '車', "くるま".to_string())
    }

    fn reading_kun3() -> Reading {
        Reading::new(ReadingType::Kunyomi, '大', "-おお.いに".to_string())
    }

    #[test]
    fn test_reading() {
        let on1 = reading_on1();
        let kun1 = reading_kun();
        let kun2 = reading_kun2();
        let kun3 = reading_kun3();
        let readings = &[on1, kun1, kun2, kun3];

        let formatted = &["長", "長い", "車", "大いに"];
        for (i, r) in readings.iter().enumerate() {
            assert_eq!(r.format_reading_with_literal(), formatted[i]);
        }
    }
}
