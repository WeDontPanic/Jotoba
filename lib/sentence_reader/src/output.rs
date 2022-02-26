use crate::sentence::part::Part;

/// Result of a sentence/inflection analysis
#[derive(Debug)]
pub enum ParseResult {
    Sentence(Sentence),
    InflectedWord(Part),
    None,
}

/// A split sentence
#[derive(Debug)]
pub struct Sentence {
    parts: Vec<Part>,
}

impl Sentence {
    pub fn new(parts: Vec<Part>) -> Self {
        Self { parts }
    }
}
