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
    pub radicals: HashMap<u8, BTreeSet<ResRadical>>,
}

/// Single radical with its enabled/disabled state, representing whether it can be used together
/// with the currently picked radicals or not.
#[derive(Serialize, Deserialize, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct ResRadical {
    #[serde(rename = "l")]
    pub literal: char,
}
