use serde::{Deserialize, Serialize};

use super::{information::Information, priority::Priority};

/// A single dictionary entry representing a words reading
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, Hash, Eq)]
pub struct Dict {
    pub reading: String,
    pub kanji: bool,
    pub no_kanji: bool,
    pub priorities: Option<Vec<Priority>>,
    pub reading_info: Option<Vec<Information>>,
    pub is_main: bool,
}

impl Dict {
    /// Returns the length of the dictionaries reading
    #[inline]
    pub fn len(&self) -> usize {
        // TODO: use proper len calculation here
        self.reading.chars().count()
    }

    /// Returns `true` if the reading has a length of zero
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.reading.is_empty()
    }
}
