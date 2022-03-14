mod analyzer;
mod grammar;
pub mod output;
mod sentence;

use once_cell::sync::Lazy;
use output::ParseResult;
use sentence::SentenceAnalyzer;

pub use igo_unidic;

pub use output::Sentence;
pub use sentence::part::Part;

/// The path of the unidict-mecab dictionary
pub const NL_PARSER_PATH: &str = "./unidic-mecab";

/// A global natural language parser
pub static JA_NL_PARSER: Lazy<igo_unidic::Parser> =
    Lazy::new(|| igo_unidic::Parser::new(NL_PARSER_PATH).unwrap());

/// Parser for sentence
pub struct Parser<'input> {
    sentence_analyzer: SentenceAnalyzer<'input>,
}

impl<'input> Parser<'input> {
    /// Creates a new InputTextParser
    pub fn new(original: &'input str) -> Self {
        let sentence_analyzer = SentenceAnalyzer::new(
            analyzer::get_grammar_analyzer(),
            JA_NL_PARSER.parse(original),
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
