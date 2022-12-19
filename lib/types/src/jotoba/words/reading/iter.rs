use super::Reading;
use crate::jotoba::words::Dict;

/// Iterator over all readings of a word
pub struct ReadingIter<'a> {
    reading: &'a Reading,
    allow_kana: bool,
    did_kanji: bool,
    did_kana: bool,
    alternative_pos: u8,
}

impl<'a> ReadingIter<'a> {
    #[inline]
    pub(crate) fn new(reading: &'a Reading, allow_kana: bool) -> Self {
        Self {
            reading,
            allow_kana,
            did_kana: false,
            did_kanji: false,
            alternative_pos: 0,
        }
    }
}

impl<'a> Iterator for ReadingIter<'a> {
    type Item = &'a Dict;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.did_kana && self.allow_kana {
            self.did_kana = true;
            return Some(&self.reading.kana);
        }
        if !self.did_kanji && self.reading.kanji.is_some() {
            self.did_kanji = true;
            return Some(self.reading.kanji.as_ref().unwrap());
        }
        let i = self
            .reading
            .alternative
            .get(self.alternative_pos as usize)?;
        self.alternative_pos += 1;
        Some(i)
    }
}
