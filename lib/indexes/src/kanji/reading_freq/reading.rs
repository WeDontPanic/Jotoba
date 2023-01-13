use serde::{Deserialize, Serialize};

/// Reading and its frequency
#[derive(Serialize, Deserialize, Debug, Default, PartialEq, Ord, Eq)]
pub struct ReadingFreq {
    pub reading: String,
    pub count: u32,
}

impl ReadingFreq {
    /// Creates a new Reading
    #[inline]
    pub fn new(reading: String) -> Self {
        Self { reading, count: 0 }
    }

    /// Increment the reading
    #[inline]
    pub fn inc(&mut self, c: u32) {
        self.count += c;
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.count == 0
    }
}

impl PartialOrd for ReadingFreq {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.count.partial_cmp(&other.count)
    }
}
