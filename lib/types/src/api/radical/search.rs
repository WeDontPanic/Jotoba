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
}
