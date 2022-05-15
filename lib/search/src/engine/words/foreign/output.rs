use types::jotoba::words::Word;

#[derive(Hash, PartialEq, Eq, Clone)]
pub struct WordOutput {
    pub word: &'static Word,
    pub positions: Vec<u16>,
}

impl WordOutput {
    #[inline]
    pub(crate) fn new(word: &'static Word, positions: Vec<u16>) -> Self {
        Self { word, positions }
    }
}

impl AsRef<Word> for WordOutput {
    #[inline]
    fn as_ref(&self) -> &Word {
        &self.word
    }
}
