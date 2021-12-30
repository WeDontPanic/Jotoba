use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// Request struct for kanji_by_radicals endpoint
#[derive(Deserialize)]
pub struct RadicalsRequest {
    pub radicals: Vec<char>,
}

/// Response struct for kanji_by_radicals endpoint
#[derive(Serialize)]
pub struct RadicalsResponse {
    pub kanji: HashMap<i32, Vec<char>>,
    pub possible_radicals: Vec<char>,
}
