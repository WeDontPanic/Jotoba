use crate::models::kanji::{Kanji, Radical};

use super::ResourceStorage;
use sorted_intersection::SortedIntersection;

#[derive(Clone, Copy)]
pub struct KanjiRetrieve<'a> {
    storage: &'a ResourceStorage,
}

impl<'a> KanjiRetrieve<'a> {
    #[inline]
    pub(super) fn new(storage: &'a ResourceStorage) -> Self {
        KanjiRetrieve { storage }
    }

    /// Returns an iterator over all loaded kanji
    #[inline]
    pub fn all(&self) -> impl Iterator<Item = &Kanji> {
        self.storage.dict_data.kanji.iter().map(|i| i.1)
    }

    /// Get a kanji by its sequence id
    #[inline]
    pub fn by_literal(&self, literal: char) -> Option<&Kanji> {
        self.storage.dict_data.kanji.get(&literal)
    }

    /// Returns all kanji with the given radicals
    #[inline]
    pub fn by_radicals(&self, radicals: &[char]) -> Vec<&Kanji> {
        let rad_map = &self.storage.dict_data.rad_map;

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

    /// Returns an iterator over all radicals
    #[inline]
    pub fn radicals(&self) -> impl Iterator<Item = &Radical> {
        self.storage.dict_data.radicals.iter().map(|i| i.1)
    }

    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = &Kanji> {
        self.storage.dict_data.kanji.iter().map(|i| i.1)
    }
}
