/// A kanji-reading search
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct KanjiReading {
    /// The provided kanji literal
    pub literal: char,
    /// The provided kanji reading
    pub reading: String,
}

impl KanjiReading {
    pub fn new(literal: &str, reading: &str) -> KanjiReading {
        KanjiReading {
            literal: literal.chars().next().unwrap(),
            reading: reading.to_string(),
        }
    }
}
