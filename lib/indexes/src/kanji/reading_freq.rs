use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use types::jotoba::kanji::Kanji;

#[derive(Serialize, Deserialize)]
pub struct FrequencyIndex {
    pub data: HashMap<char, FreqData>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FreqData {
    pub total: usize,
    pub readings: Vec<Reading>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Reading {
    pub reading: String,
    pub count: u32,
}

impl Reading {
    pub fn new(reading: String, count: u32) -> Self {
        Self { reading, count }
    }
}

impl FreqData {
    pub fn new(readings: Vec<String>) -> Self {
        let readings = readings
            .into_iter()
            .map(|i| Reading::new(i, 0))
            .collect::<Vec<_>>();
        Self { total: 0, readings }
    }

    #[inline]
    pub fn total(&self) -> usize {
        self.total
    }

    #[inline]
    pub fn get_reading<S: AsRef<str>>(&self, r: S) -> Option<u32> {
        self.readings
            .iter()
            .find(|i| i.reading == r.as_ref())
            .map(|i| i.count)
    }
}

impl FrequencyIndex {
    /// Returns a FreqData for the kanji `c`
    #[inline]
    pub fn get(&self, c: char) -> Option<&FreqData> {
        self.data.get(&c)
    }

    pub fn new(all_kanji: &[Kanji]) -> FrequencyIndex {
        let mut data = HashMap::new();

        for kanji in all_kanji {
            let mut readings = vec![];

            let on = kanji.onyomi.clone();
            let kun = kanji.kunyomi.clone();

            for reading in on.into_iter().chain(kun.into_iter()) {
                readings.push(reading);
            }

            data.insert(kanji.literal, FreqData::new(readings));
        }

        FrequencyIndex { data }
    }

    /// Returns the normalized frequency for `reading`
    #[inline]
    pub fn norm_reading_freq(&self, kanji: char, reading: &str) -> Option<f32> {
        let freq_data = self.data.get(&kanji)?;
        let read_freq = freq_data.get_reading(reading)?;
        Some(read_freq as f32 / freq_data.total as f32)
    }
}
