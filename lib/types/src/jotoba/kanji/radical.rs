use serde::{Deserialize, Serialize};

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
