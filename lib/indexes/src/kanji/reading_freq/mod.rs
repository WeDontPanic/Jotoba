pub mod k_freq_item;
pub mod reading;

use self::k_freq_item::KFreqItem;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use types::jotoba::kanji::Kanji;

/// An index that can hold kanji along with their various readings which this lovely language
/// 'supports'. Each reading entry has a frequency assigned how often it occurrs for the given
/// Kanji.
#[derive(Serialize, Deserialize)]
pub struct FrequencyIndex {
    pub data: HashMap<char, KFreqItem>,
}

impl FrequencyIndex {
    /// Create a new FrequencyIndex with a given set of kanji that will be supported
    pub fn new(all_kanji: &[Kanji]) -> FrequencyIndex {
        let mut data = HashMap::new();

        for kanji in all_kanji {
            let mut readings = vec![];

            let on = kanji.onyomi.clone();
            let kun = kanji.kunyomi.clone();

            for reading in on.into_iter().chain(kun.into_iter()) {
                readings.push(reading);
            }

            data.insert(kanji.literal, KFreqItem::new(readings));
        }

        FrequencyIndex { data }
    }

    /// Inserts a new reading for the given kanji. All readings of the kanji for those `matches`
    /// returns `true` will be incremented
    pub fn add_reading<F>(&mut self, kanji_lit: char, matches: F) -> bool
    where
        F: Fn(&str) -> bool,
    {
        let entry = match self.data.get_mut(&kanji_lit) {
            Some(s) => s,
            None => return false,
        };

        let c = entry
            .readings
            .iter_mut()
            .filter(|i| matches(&i.reading))
            .map(|i| i.count += 1)
            .count();

        if c == 0 {
            return false;
        }

        // We're passing one reading. If there are multiple entries for one single entry,
        // they're treated equally, so we're counting up all matches but only counting one
        // total
        entry.inc_total(1);

        true
    }

    /// Removes all empty items from the index
    pub fn clear(&mut self) {
        self.data.retain(|_, v| !v.is_empty());
    }

    /// Returns a FreqData for the kanji `c`
    #[inline]
    pub fn get(&self, c: char) -> Option<&KFreqItem> {
        self.data.get(&c)
    }

    /// Returns the normalized frequency for `reading`
    #[inline]
    pub fn norm_reading_freq(&self, kanji: char, reading: &str) -> Option<f32> {
        self.norm_reading_freq_th(kanji, reading, 200)
    }

    /// Returns the normalized frequency for `reading`
    #[inline]
    pub fn norm_reading_freq_th(&self, kanji: char, reading: &str, th: usize) -> Option<f32> {
        let freq_data = self.data.get(&kanji)?;
        let read_freq = freq_data.get_reading(reading)?.count;
        if freq_data.total() < th {
            return None;
        }
        Some(read_freq as f32 / freq_data.total() as f32)
    }
}
