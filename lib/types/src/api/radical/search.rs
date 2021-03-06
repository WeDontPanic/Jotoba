use serde::{Deserialize, Serialize};
use std::collections::{BTreeSet, HashMap};

/// Request struct for kanji_by_radicals endpoint
#[derive(Deserialize)]
pub struct Request {
    pub query: String,
}

/// Response struct for kanji_by_radicals endpoint
#[derive(Serialize, Deserialize, Default)]
pub struct Response {
    pub radicals: HashMap<u8, BTreeSet<char>>,
    pub kanji: Vec<KanjiRads>,
}

/// Kanji literal with radicals
#[derive(Serialize, Deserialize, Default, PartialEq, Eq)]
pub struct KanjiRads {
    pub kanji: char,
    pub rads: HashMap<u32, Vec<char>>,
}

impl KanjiRads {
    #[inline]
    pub fn new(kanji: char, rads: HashMap<u32, Vec<char>>) -> Self {
        Self { kanji, rads }
    }
}
