use igo_unidic::{Conjungation, Morpheme, WordClass};

#[derive(Clone, Debug, PartialEq)]
pub struct OwnedMorpheme<'dict> {
    pub surface: String,
    pub basic: &'dict str,
    pub word_class: WordClass<'dict>,
    pub conjungation: Conjungation,
    pub reading: &'dict str,
    pub lexeme: &'dict str,
    pub start: usize,
}

impl<'dict> From<Morpheme<'dict, '_>> for OwnedMorpheme<'dict> {
    #[inline]
    fn from(m: Morpheme<'dict, '_>) -> Self {
        Self {
            surface: m.surface.to_string(),
            basic: m.basic,
            word_class: m.word_class,
            conjungation: m.conjungation,
            reading: m.reading,
            lexeme: m.lexeme,
            start: m.start,
        }
    }
}

impl<'dict> OwnedMorpheme<'dict> {
    /// Gets the main lexeme. Falls back on surface if lexeme is empty
    pub fn reading(&self) -> &str {
        if !self.lexeme.is_empty() {
            self.lexeme
        } else {
            &self.surface
        }
    }
}
