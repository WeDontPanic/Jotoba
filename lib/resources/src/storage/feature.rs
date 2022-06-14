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

    // Words
    WordIrregularIchidan,
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
