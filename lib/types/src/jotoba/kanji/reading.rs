use japanese::JapaneseExt;

use super::Kanji;

/// ReadingType of a kanji's reading. `Kunyomi` represents japanese readings and `Onyomi`
/// represents original chinese readings.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ReadingType {
    Kunyomi,
    Onyomi,
}

#[derive(Clone, Debug)]
pub struct Reading {
    r_type: ReadingType,
    literal: char,
    inner: String,
}

impl Reading {
    pub(crate) fn new(r_type: ReadingType, literal: char, inner: String) -> Self {
        Reading {
            r_type,
            literal,
            inner,
        }
    }

    /// Get the reading's r type.
    #[inline]
    pub fn get_type(&self) -> ReadingType {
        self.r_type
    }

    /// Get a mutable reference to the reading's literal.
    #[inline]
    pub fn get_literal(&self) -> &char {
        &self.literal
    }

    /// Get a reference to the reading's inner.
    #[inline]
    pub fn get_raw(&self) -> &str {
        self.inner.as_ref()
    }

    /// Returns a string with the reading and literal merged. If the reading is an onyomi reading,
    /// this is equal to the literal. For kunyomi readings this can be an example: (inner: "だま.る") => "黙る".
    /// This also formats the reading to hiragana
    pub fn format_reading_with_literal(&self) -> String {
        match self.r_type {
            ReadingType::Kunyomi => {
                let r = if self.inner.contains('.') {
                    let right = self.inner.split('.').nth(1).unwrap_or_default();
                    format!("{}{}", self.literal, right)
                } else {
                    self.literal.to_string()
                };
                r.replace("-", "")
            }
            ReadingType::Onyomi => self.literal.to_hiragana(),
        }
    }

    /// Returns `true` if `kanji` has this reading
    #[inline]
    pub fn matches_kanji(&self, kanji: &Kanji) -> bool {
        self.literal == kanji.literal && kanji.has_reading(&self.inner)
    }

    /// Returns the literal as newly allocated `String`
    #[inline]
    pub fn get_lit_str(&self) -> String {
        self.get_literal().to_string()
    }

    /// Returns `true` if the literal captures the entire literal
    #[inline]
    pub fn is_full_reading(&self) -> bool {
        !self.inner.contains('-') && !self.inner.contains('.')
    }
}

impl PartialEq<ReadingType> for &Reading {
    #[inline]
    fn eq(&self, other: &ReadingType) -> bool {
        self.r_type == *other
    }
}

/// A kanji-reading search item
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct ReadingSearch {
    /// The provided kanji literal
    pub literal: char,
    /// The provided kanji reading
    pub reading: String,
}

impl ReadingSearch {
    #[inline]
    pub fn new(literal: &str, reading: &str) -> Self {
        ReadingSearch {
            literal: literal.chars().next().unwrap(),
            reading: reading.to_string(),
        }
    }
}
