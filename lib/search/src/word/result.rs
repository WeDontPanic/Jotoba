use types::jotoba::{
    kanji::Kanji,
    words::{inflection::Inflection, Word},
};

#[derive(Debug, Clone, PartialEq)]
pub struct WordResult {
    pub items: Vec<Item>,
    pub count: usize,
    pub contains_kanji: bool,
    pub inflection_info: Option<InflectionInformation>,
    pub sentence_parts: Option<sentence_reader::Sentence>,
    pub sentence_index: usize,
    pub searched_query: String,
}

impl WordResult {
    #[inline]
    pub fn has_word(&self) -> bool {
        self.items.iter().any(|i| i.is_word())
    }

    /// Returns all words and kanji split in two separate lists
    pub fn get_items(&self) -> (Vec<&Word>, Vec<&Kanji>) {
        let mut words = vec![];
        let mut kanjis = vec![];

        for item in &self.items {
            match item {
                Item::Word(word) => words.push(word),
                Item::Kanji(kanji) => kanjis.push(kanji),
            }
        }

        (words, kanjis)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct InflectionInformation {
    /// Normalized form of the word
    pub lexeme: String,
    /// All inflections
    pub inflections: Vec<Inflection>,
}

impl InflectionInformation {
    pub fn from_part(part: &sentence_reader::Part) -> Option<Self> {
        if !part.has_inflections() {
            return None;
        }

        Some(InflectionInformation {
            lexeme: part.get_normalized(),
            inflections: part.inflections().to_vec(),
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Item {
    Word(Word),
    Kanji(Kanji),
}

impl Item {
    /// Returns `true` if the item is [`Word`].
    #[inline]
    pub fn is_word(&self) -> bool {
        matches!(self, Self::Word(..))
    }

    /// Returns `true` if the item is [`Kanji`].
    #[inline]
    pub fn is_kanji(&self) -> bool {
        matches!(self, Self::Kanji(..))
    }
}

impl From<Kanji> for Item {
    #[inline]
    fn from(k: Kanji) -> Self {
        Self::Kanji(k)
    }
}

impl From<Word> for Item {
    #[inline]
    fn from(w: Word) -> Self {
        Self::Word(w)
    }
}

pub fn selected(curr: usize, selected: usize) -> &'static str {
    if curr == selected {
        "selected"
    } else {
        ""
    }
}
