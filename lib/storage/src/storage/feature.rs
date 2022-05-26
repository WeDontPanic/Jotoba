#[derive(Clone, Copy, PartialEq, Eq, Debug)]
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

    // Kanji
    GenkiTags,
}
