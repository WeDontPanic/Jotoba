mod analyzer;
mod grammar;
pub mod output;
mod sentence;

use std::path::Path;

use once_cell::sync::{Lazy, OnceCell};
use output::ParseResult;
use sentence::SentenceAnalyzer;

pub use igo_unidic;

pub use output::Sentence;
pub use sentence::part::Part;

pub static JA_NL_PARSER: Lazy<OnceCell<igo_unidic::Parser>> = Lazy::new(|| OnceCell::new());

pub fn load_parser<P: AsRef<Path>>(path: P) {
    let parser = igo_unidic::Parser::new(path.as_ref().to_str().unwrap()).unwrap();
    JA_NL_PARSER.set(parser).ok();
}

pub fn wait() {
    JA_NL_PARSER.wait();
}

pub fn is_loaded() -> bool {
    JA_NL_PARSER.get().is_some()
}

/// Parser for sentence
pub struct Parser<'input> {
    sentence_analyzer: SentenceAnalyzer<'input>,
}

impl<'input> Parser<'input> {
    /// Creates a new InputTextParser
    pub fn new(original: &'input str) -> Self {
        let sentence_analyzer = SentenceAnalyzer::new(
            analyzer::get_grammar_analyzer(),
            JA_NL_PARSER.get().unwrap().parse(original),
        );

        Self { sentence_analyzer }
    }

    /// Execute the parsing
    pub fn parse(&self) -> ParseResult {
        let mut sent_parse = self.sentence_analyzer.analyze::<Part>();

        if sent_parse.is_empty() {
            return ParseResult::None;
        } else if sent_parse.len() == 1 {
            let parsed = sent_parse.remove(0);
            return parsed
                .has_inflections()
                .then(|| ParseResult::InflectedWord(parsed))
                .unwrap_or(ParseResult::None);
        }

        let sentence = Sentence::new(sent_parse);
        ParseResult::Sentence(sentence)
    }
}
