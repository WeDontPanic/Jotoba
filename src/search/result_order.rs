use std::cmp::Ordering;

use super::{result::word::Item, SearchMode};
use crate::{japanese::JapaneseExt, parse::jmdict::languages::Language};

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
        let this_has_reading_o = this.has_reading(self.query, true);

        // Show common items at the top
        let this_is_common = this.is_common() && !other.is_common();
        let other_is_common = other.is_common() && !this.is_common();
        // Show exact readings at the top
        let this_has_reading = this_has_reading_o && !other_has_reading;
        let other_has_reading = !this_has_reading_o && other_has_reading;

        let this_is_exact_reading = self.is_exact_reading(this);
        let other_is_exact_reading = self.is_exact_reading(other);

        if other_is_common
            || (other_is_exact_reading && !this_is_exact_reading)
            || other_has_reading
        {
            Ordering::Greater
        } else if this_is_common || this_has_reading || this_is_exact_reading {
            // Show directly matching and common items at the top
            Ordering::Less
        } else if this.reading.kana.is_some() && other.reading.kana.is_some() {
            // If both have a kana reading
            let self_read = this.reading.kana.as_ref().unwrap();
            let other_read = other.reading.kana.as_ref().unwrap();

            // Order by length,
            // shorter words will be displayed first
            if self_read.len() < other_read.len() {
                Ordering::Less
            } else if self_read.len() > other_read.len() {
                Ordering::Greater
            } else {
                if this.reading.get_jplt_lvl().is_some() && other.reading.get_jplt_lvl().is_none() {
                    Ordering::Less
                } else if this.reading.get_jplt_lvl().is_none()
                    && other.reading.get_jplt_lvl().is_some()
                {
                    Ordering::Greater
                } else {
                    Ordering::Equal
                }
            }
        } else {
            // In case one word doesn't have a
            // kana reading, both are handled
            // equally... shouldn't happen though
            Ordering::Equal
        }
    }

    fn is_exact_reading(&self, this: &Item) -> bool {
        if self.query.has_kanji() && this.reading.kanji.is_none() {
            return false;
        }

        if self.query.has_kanji() {
            this.reading.kanji.as_ref().unwrap().reading == self.query
        } else {
            this.reading.kanji.is_none()
                && this.reading.get_reading().reading.is_japanese()
                && this.has_reading(self.query, true)
        }
    }
}

/// Represents the ordering for result based on
/// foreign (not japanese) search-input
pub(crate) struct GlossWordOrder<'a> {
    query: &'a str,
}

impl<'a> GlossWordOrder<'a> {
    pub(crate) fn new(query: &'a str) -> Self {
        Self { query }
    }

    /// Sort native words results
    pub(crate) fn sort(&self, vec: &mut Vec<Item>) {
        vec.sort_by(|a, b| self.native_words(a, b))
    }

    /// Returns an Ordering variant based on the input items
    fn native_words(&self, this: &Item, other: &Item) -> Ordering {
        let this_exact_l = self.calc_likelienes(this, SearchMode::Exact, true);
        let other_exact_l = self.calc_likelienes(other, SearchMode::Exact, true);

        if this.is_katakana_word() && !other.is_katakana_word() {
            return Ordering::Greater;
        } else if !this.is_katakana_word() && other.is_katakana_word() {
            return Ordering::Less;
        }

        if this_exact_l == 100 && other_exact_l < 100 {
            return Ordering::Less;
        }

        // Show single translations more on top
        if this_exact_l == 100 && other_exact_l == 100 {
            if let Some(this_language) = self.find_lang(this) {
                if let Some(other_language) = self.find_lang(other) {
                    let this_l_sense = this.senses_by_lang(this_language);
                    let other_l_sense = other.senses_by_lang(other_language);
                    if this_l_sense.is_some() && other_l_sense.is_some() {
                        if this_l_sense.unwrap().len() == 1 && other_l_sense.unwrap().len() > 1 {
                            return Ordering::Less;
                        }
                    }
                }
            }
        }

        if this.is_common() && !other.is_common() {
            return Ordering::Less;
        }

        if this.reading.get_jplt_lvl().is_some() && other.reading.get_jplt_lvl().is_none() {
            return Ordering::Less;
        } else if this.reading.get_jplt_lvl().is_none() && other.reading.get_jplt_lvl().is_some() {
            return Ordering::Greater;
        }

        Ordering::Equal
    }

    /*
    pub fn get_likelynes(&self, this: &Item) -> u8 {
        let l_exact = self.calc_likelynes(this, SearchMode::Exact, false);
        let l_exact_icase = self.calc_likelynes(this, SearchMode::Exact, true) / 3;
        let l_contains = self.calc_likelynes(this, SearchMode::Variable, true) / 10;
        println!("exact: {}", l_exact);
        println!("exact_icase: {}", l_exact_icase);
        println!("contains: {}", l_contains);

        if l_exact > 0 {
            return l_exact;
        }

        if l_exact_icase > 0 {
            return l_exact_icase;
        }

        if l_contains > 0 {
            return l_contains;
        }

        0
    }
    */

    pub fn calc_likelienes(&self, this: &Item, s_mode: SearchMode, ign_case: bool) -> u8 {
        let n: usize = this.senses.iter().map(|i| i.glosses.iter().count()).sum();
        let pos = Self::get_query_pos_in_gloss(&self, this, s_mode, ign_case);
        if pos.is_none() {
            return 0;
        }
        100 - Self::calc_importance(pos.unwrap(), n) as u8
    }

    pub fn find_lang(&self, this: &Item) -> Option<Language> {
        self.get_lang(this, SearchMode::Exact, false)
            .map(|i| Some(i))
            .unwrap_or(
                self.get_lang(this, SearchMode::Exact, true)
                    .map(|i| Some(i))
                    .unwrap_or(self.get_lang(this, SearchMode::Variable, true)),
            )
    }

    pub fn get_lang(&self, this: &Item, s_mode: SearchMode, ign_case: bool) -> Option<Language> {
        let items = this.get_senses();

        for lang_senes in items.iter() {
            for sense in lang_senes {
                for gloss in sense.glosses.iter() {
                    if s_mode.str_eq(gloss.gloss.as_str(), self.query, ign_case) {
                        return Some(sense.language);
                    }
                }
            }
        }

        None
    }

    pub fn get_query_pos_in_gloss(
        &self,
        this: &Item,
        s_mode: SearchMode,
        ign_case: bool,
    ) -> Option<usize> {
        let items = this.get_senses();

        for lang_senes in items.iter() {
            let mut pos = 0;
            for sense in lang_senes {
                for gloss in sense.glosses.iter() {
                    if s_mode.str_eq(gloss.gloss.as_ref(), self.query, ign_case) {
                        return Some(pos);
                    }
                    pos += 1;
                }
            }
        }

        None
    }

    /// Returns a value from 1 to 100 based on importance
    /// an item inside a result
    fn calc_importance(pos: usize, total: usize) -> usize {
        (pos * 100) / total
    }

    fn is_exact_reading(&self, this: &Item) -> bool {
        if self.query.has_kanji() && this.reading.kanji.is_none() {
            return false;
        }

        if self.query.has_kanji() {
            this.reading.kanji.as_ref().unwrap().reading == self.query
        } else {
            this.reading.kanji.is_none()
                && this.reading.get_reading().reading.is_japanese()
                && this.has_reading(self.query, true)
        }
    }
}

#[cfg(test)]
mod test {
    use super::GlossWordOrder;
    use super::Item;
    use crate::search::result::word::{Gloss, Sense};
    use crate::search::SearchMode;

    fn str_to_gloss(values1_sense: Vec<&str>) -> Vec<Gloss> {
        values1_sense
            .into_iter()
            .map(|i| Gloss {
                gloss: i.to_owned(),
                ..Default::default()
            })
            .collect::<Vec<_>>()
    }

    fn make_word1_item(values1_sense: Vec<&str>) -> Item {
        let glosses1 = str_to_gloss(values1_sense);
        Item {
            senses: vec![Sense {
                glosses: glosses1,
                ..Default::default()
            }],
            ..Item::default()
        }
    }

    fn make_word2_item(values1_sense: Vec<&str>, values2_sense: Vec<&str>) -> Item {
        let glosses1 = str_to_gloss(values1_sense);
        let glosses2 = str_to_gloss(values2_sense);

        Item {
            senses: vec![
                Sense {
                    glosses: glosses1,
                    ..Default::default()
                },
                Sense {
                    glosses: glosses2,
                    ..Default::default()
                },
            ],
            ..Item::default()
        }
    }

    #[test]
    fn test_calc_likeliness_char() {
        let search = GlossWordOrder { query: "c" };

        let item_a = make_word2_item(vec!["a", "b"], vec!["c"]);
        let item_b = make_word2_item(vec!["c", "b"], vec!["0"]);
        let item_c = make_word2_item(vec!["b", "c"], vec!["0"]);

        /*
        assert_eq!(search.get_likelynes(&item_a), 34);
        assert_eq!(search.get_likelynes(&item_b), 100);
        assert_eq!(search.get_likelynes(&item_c), 67);
        */
    }

    #[test]
    fn test_calc_likeliness_word_1() {
        let search = GlossWordOrder { query: "hello" };

        let item_a = make_word2_item(vec!["good day", "hello"], vec!["good afternoon"]);
        let item_b = make_word1_item(vec!["Hello", "good day"]);
        let item_c = make_word2_item(vec!["hello", "good day"], vec!["bye"]);

        assert_eq!(
            search.calc_likelienes(&item_a, SearchMode::Exact, false),
            67
        );
        assert_eq!(search.calc_likelienes(&item_a, SearchMode::Exact, true), 67);

        assert_eq!(search.calc_likelienes(&item_b, SearchMode::Exact, false), 0);

        assert_eq!(
            search.calc_likelienes(&item_c, SearchMode::Exact, true),
            100
        );
    }
}
