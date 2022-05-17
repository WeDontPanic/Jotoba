use types::jotoba::words::Word;

#[derive(Hash, Eq, Clone)]
pub struct WordOutput {
    pub word: &'static Word,
    pub positions: Vec<u16>,
}

impl PartialEq for WordOutput {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.word == other.word
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
            let (sense, gloss) = types::jotoba::words::sense::from_unique_id(*i);
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
