use std::cmp::Ordering;

use super::result::word::Item;
use crate::japanese::JapaneseExt;

/// Represents the ordering for result based on
/// native search-input
pub(crate) struct NativeWordOrder<'a> {
    query: &'a str,
}

impl<'a> NativeWordOrder<'a> {
    pub(crate) fn new(query: &'a str) -> Self {
        Self { query }
    }

    /// Sort native words results
    pub(crate) fn sort(&self, vec: &mut Vec<Item>) {
        vec.sort_by(|a, b| self.native_words(a, b))
    }

    /// Returns an Ordering variant based on the input items
    fn native_words(&self, this: &Item, other: &Item) -> Ordering {
        let other_has_reading = other.has_reading(self.query, true);
        if this.sequence == 1000520 {
            println!("{}", this.sequence);
        }

        // Show common items at the top
        let this_is_common = this.is_common() && !other.is_common() && !other_has_reading;
        // Show exact readings at the top
        let this_has_reading = this.has_reading(self.query, true) && !other_has_reading;
        let is_exact_reading = this.reading.kanji.is_none(); //&& this.reading.get_reading().reading.is_japanese(); // && this.has_reading(self.query, true);

        // Show directly matching and common items at the top
        if this_is_common || this_has_reading || is_exact_reading {
            Ordering::Less
        } else if this.reading.kana.is_some() && other.reading.kana.is_some() {
            // If both have a kana reading
            let self_read = this.reading.get_reading();
            let other_read = other.reading.get_reading();

            // Order by length,
            // shorter words will be displayed first
            if self_read.len() < other_read.len() {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        } else {
            // In case one word doesn't have a
            // kana reading, both are handled
            // equally... shouldn't happen though
            Ordering::Equal
        }
    }
}

// TODO add GlossWordOrder
