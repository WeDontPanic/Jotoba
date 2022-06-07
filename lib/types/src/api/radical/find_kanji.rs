use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Request struct for kanji_by_radicals endpoint
#[derive(Deserialize)]
pub struct Request {
    pub radicals: Vec<char>,
}

/// Response struct for kanji_by_radicals endpoint
#[derive(Serialize, Deserialize)]
pub struct Response {
    pub kanji: HashMap<u32, Vec<char>>,
    pub possible_radicals: Vec<char>,
}
