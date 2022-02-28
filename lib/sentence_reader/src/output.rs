use crate::sentence::part::Part;

/// Result of a sentence/inflection analysis
#[derive(Debug)]
pub enum ParseResult {
    Sentence(Sentence),
    InflectedWord(Part),
    None,
}

/// A split sentence
#[derive(Debug, Clone, PartialEq)]
pub struct Sentence {
    parts: Vec<Part>,
}

impl Sentence {
    pub fn new(parts: Vec<Part>) -> Self {
        Self { parts }
    }

    /// Returns word at `pos`
    pub fn get_at(&self, pos: usize) -> Option<&Part> {
        self.parts.get(pos)
    }

    /// Returns word at `pos`
    pub fn get_at_mut(&mut self, pos: usize) -> Option<&mut Part> {
        self.parts.get_mut(pos)
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Part> {
        self.parts.iter_mut()
    }

    pub fn iter(&self) -> impl Iterator<Item = &Part> {
        self.parts.iter()
    }

    /// returns amount of words
    pub fn word_count(&self) -> usize {
        self.parts.len()
    }
}
