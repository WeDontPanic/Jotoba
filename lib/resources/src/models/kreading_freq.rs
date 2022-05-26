use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct FrequencyIndex {
    pub data: HashMap<char, FreqData>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FreqData {
    pub total: usize,
    pub readings: Vec<(String, u32)>,
}
