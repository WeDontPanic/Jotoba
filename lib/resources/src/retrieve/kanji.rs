use sorted_intersection::SortedIntersection;
use types::jotoba::kanji::{radical::DetailedRadical, Kanji};

use super::super::storage::kanji::KanjiStorage;

#[derive(Clone, Copy)]
pub struct KanjiRetrieve<'a> {
    storage: &'a KanjiStorage,
}

impl<'a> KanjiRetrieve<'a> {
    #[inline(always)]
    pub(crate) fn new(storage: &'a KanjiStorage) -> Self {
        KanjiRetrieve { storage }
    }

    /// Get a kanji by its sequence id
    #[inline]
    pub fn by_literal(&self, literal: char) -> Option<&Kanji> {
        //self.storage.dict_data.kanji.kanji.get(&literal)
        self.storage.literal_index.get(&literal)
    }

    /// Returns all kanji with the given radicals
    #[inline]
    pub fn by_radicals(&self, radicals: &[char]) -> Vec<&Kanji> {
        let rad_map = &self.storage.radical_map;

        let mut maps = radicals
            .iter()
            .filter_map(|i| rad_map.get(i).map(|i| i.iter()))
            .collect::<Vec<_>>();

        if maps.is_empty() {
            return vec![];
        }

        SortedIntersection::new(&mut maps)
            .filter_map(|i| self.by_literal(*i))
            .collect::<Vec<_>>()
    }

    /// Returns all kanji with given jlpt level
    #[inline]
    pub fn by_jlpt(&self, jlpt: u8) -> Option<&Vec<char>> {
        self.storage.jlpt_data.get(&jlpt)
    }

    /// Returns an iterator over all radicals
    #[inline]
    pub fn radicals(&self) -> impl Iterator<Item = &DetailedRadical> {
        self.storage.radical_data.iter().map(|i| i.1)
    }

    /// Returns a list of kanji taught in given genki_lesson
    #[inline]
    pub fn by_genki_lesson(&self, genki_lektion: u8) -> Option<&Vec<char>> {
        self.storage.genki_levels.get(&genki_lektion)
    }

    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = &Kanji> {
        self.storage.literal_index.iter().map(|i| i.1)
    }

    #[inline]
    pub fn all(&self) -> Vec<Kanji> {
        self.iter().cloned().collect()
    }

    /// Returns the count of kanji
    #[inline]
    pub fn count(&self) -> usize {
        self.storage.literal_index.len()
    }
}
