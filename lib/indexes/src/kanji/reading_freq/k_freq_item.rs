use super::reading::ReadingFreq;
use serde::{Deserialize, Serialize};

/// All reading data for a single kanji
#[derive(Serialize, Deserialize, Debug)]
pub struct KFreqItem {
    pub readings: Vec<ReadingFreq>,
    pub total: usize,
}

impl KFreqItem {
    /// Creates a new Kanji frequency item with the provided readings
    pub fn new(readings: Vec<String>) -> Self {
        let readings = readings
            .into_iter()
            .map(|i| ReadingFreq::new(i))
            .collect::<Vec<_>>();
        Self { readings, total: 0 }
    }

    /// Get the total amount of counted readings for a kanji
    #[inline]
    pub fn total(&self) -> usize {
        self.total
    }

    /// Increase the total value of counted readings for a kanji
    #[inline]
    pub fn inc_total(&mut self, add: usize) {
        self.total += add
    }

    /// Returns `true` if the kanji readings are completely empty
    pub fn is_empty(&self) -> bool {
        self.readings.is_empty() || (self.readings.iter().all(|i| i.is_empty()) && self.total == 0)
    }

    /// Gets all reading freq items that match the given matcher
    #[inline]
    pub fn get_readings<'a, F: Fn(&str) -> bool>(
        &'a self,
        r: F,
    ) -> impl Iterator<Item = &ReadingFreq> {
        self.readings.iter().filter(move |i| r(&i.reading))
    }

    /// Gets a reading freq item with the given string
    #[inline]
    pub fn get_reading<S: AsRef<str>>(&self, s: S) -> Option<&ReadingFreq> {
        self.readings.iter().find(|i| i.reading == s.as_ref())
    }
}
