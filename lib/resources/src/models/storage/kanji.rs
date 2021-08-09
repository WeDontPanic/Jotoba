use utilsrs::vectools::part_of;

use crate::models::kanji::Kanji;

use super::ResourceStorage;

#[derive(Clone, Copy)]
pub struct KanjiRetrieve<'a> {
    storage: &'a ResourceStorage,
}

impl<'a> KanjiRetrieve<'a> {
    #[inline]
    pub(super) fn new(storage: &'a ResourceStorage) -> Self {
        KanjiRetrieve { storage }
    }

    /// Get a kanji by its sequence id
    #[inline]
    pub fn by_literal(&self, literal: char) -> Option<&Kanji> {
        self.storage.kanji.get(&literal)
    }

    /// Returns all kanji with the given radicals
    #[inline]
    pub fn by_radicals(&self, radicals: &[char]) -> Vec<&Kanji> {
        self.storage
            .kanji
            .iter()
            .filter(|i| {
                i.1.parts
                    .as_ref()
                    .map(|i| part_of(radicals, i))
                    .unwrap_or(false)
            })
            .map(|i| i.1)
            .collect::<Vec<_>>()
    }
}
