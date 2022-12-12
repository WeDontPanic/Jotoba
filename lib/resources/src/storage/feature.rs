use strum::{EnumIter, IntoEnumIterator};

#[derive(Clone, Copy, PartialEq, Eq, Debug, EnumIter)]
pub enum Feature {
    // ----- Basic ones -----
    Words,
    Sentences,
    Names,
    Kanji,

    /// RadicalToKanji
    RadicalKanjiMap,

    /// DetailedRadicals
    RadicalData,

    // ----- Other ------

    // Sentences
    SentenceJLPT,
    SentenceTags,

    // Words
    WordIrregularIchidan,
    WordKatakana,
    WordPitch,
    SentenceAvailable,
    WordJlpt,

    // Kanji
    GenkiTags,
    SimilarKanji,
    KanjiDecompositions,
}

impl Feature {
    pub fn all() -> Vec<Feature> {
        Feature::iter().collect()
    }
}
