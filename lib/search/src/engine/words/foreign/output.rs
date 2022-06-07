use std::hash::Hash;
use types::jotoba::words::{sense, Word};

#[derive(Clone)]
pub struct WordOutput {
    pub word: &'static Word,
    pub positions: Vec<u16>,
}

impl PartialEq for WordOutput {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.word.sequence == other.word.sequence
    }
}

impl Eq for WordOutput {}

impl Hash for WordOutput {
    #[inline]
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.word.sequence.hash(state);
    }
}

impl WordOutput {
    #[inline]
    pub(crate) fn new(word: &'static Word, positions: Vec<u16>) -> Self {
        Self { word, positions }
    }

    #[inline]
    pub fn position_iter(&self) -> impl Iterator<Item = (u8, u8, u16)> + '_ {
        self.positions.iter().map(|i| {
            let (sense, gloss) = sense::from_unique_id(*i);
            (sense, gloss, *i)
        })
    }
}

impl AsRef<Word> for WordOutput {
    #[inline]
    fn as_ref(&self) -> &Word {
        &self.word
    }
}
