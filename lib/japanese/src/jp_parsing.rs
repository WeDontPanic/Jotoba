use std::cmp::Ordering;

use crate::inflection::{Inflection, SentencePart};
use error::Error;
use igo_unidic::{ConjungationForm, Morpheme, Parser, ParticleType, VerbType, WordClass};
use itertools::Itertools;
use once_cell::sync::Lazy;

pub use igo_unidic;

/// The path of the unidict-mecab dictionary
pub const NL_PARSER_PATH: &str = "./unidic-mecab";

/// A global natural language parser
pub static JA_NL_PARSER: once_cell::sync::Lazy<igo_unidic::Parser> =
    Lazy::new(|| igo_unidic::Parser::new(NL_PARSER_PATH).unwrap());

/// Potentially lexemes of inflections
pub const INFLECTION_LEXEMES: [&str; 13] = [
    "ない",    // Negative
    "ます",    // polite from
    "て",       // Te form
    "だ",       // Past short form
    "た",       // Past short form
    "です",    // Polite
    "れる",    // Passive
    "せる",    // Causative
    "られる", // Causative passive
    "な",       // na
    "ぬ",       // ン
    "で",       // some shit lol
    "たい",    // Tai form
];

pub struct InputTextParser<'dict, 'input> {
    morphemes: Vec<Morpheme<'dict, 'input>>,
    original: &'input str,
    in_db: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct WordItem<'dict, 'input> {
    pub surface: &'input str,
    pub lexeme: &'dict str,
    pub word_class: Option<WordClass<'dict>>,
    pub was_in_db: bool,
    pub inflections: Vec<Inflection>,
    pub start: usize,
    pub original_word: String,
}

impl<'dict, 'input> WordItem<'dict, 'input> {
    fn from_morpheme(
        m: Morpheme<'dict, 'input>,
        in_db: bool,
        inflections: Vec<Inflection>,
        original_word: String,
    ) -> Self {
        WordItem {
            word_class: Some(m.word_class),
            lexeme: m.lexeme,
            surface: m.surface,
            was_in_db: in_db,
            inflections,
            start: m.start,
            original_word,
        }
    }

    pub fn get_lexeme(&self) -> &str {
        if self.lexeme.is_empty() {
            self.surface
        } else {
            self.lexeme
        }
    }

    /// Converts a [`WordItem`] into a sentence part
    pub fn into_sentence_part(self, pos: i32) -> SentencePart {
        SentencePart {
            //text: self.get_lexeme().to_owned(),
            lexeme: self.get_lexeme().to_owned(),
            pos,
            info: self.word_class_to_str(),
            furigana: None,
            furi_guessed: false,
            add_class: self
                .word_class_to_str()
                .map(|i| i.to_owned().to_lowercase()),
            text: self.original_word,
        }
    }

    /// returns msgid for the current word_class or None if no word_class is set
    fn word_class_to_str(&self) -> Option<&'static str> {
        Some(match self.word_class? {
            WordClass::Particle(_) => "Particle",
            WordClass::Verb(_) => "Verb",
            WordClass::Adjective(_) => "Adjective",
            WordClass::Adverb => "Adverb",
            WordClass::Noun(_) => "Noun",
            WordClass::Pronoun => "Pronoun",
            WordClass::Interjection => "Interjection",
            WordClass::Symbol => "Symbol",
            WordClass::Conjungtion => "Conjungtion",
            WordClass::Suffix => "Suffix",
            WordClass::Prefix => "Prefix",
            WordClass::PreNoun => "Pre-noun",
            WordClass::Space => "Space",
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ParseResult<'dict, 'input> {
    pub items: Vec<WordItem<'dict, 'input>>,
}

fn inflection_from_conjungation(conj: &ConjungationForm) -> Option<Inflection> {
    Some(match conj {
        ConjungationForm::Plain => Inflection::Present,
        ConjungationForm::Imperative => Inflection::Imperative,
        ConjungationForm::Negative => Inflection::Negative,
        ConjungationForm::Conditional => Inflection::Potential,
        _ => return None,
    })
}

fn inflection_from_morpheme(
    morpheme: &Morpheme,
    main_morpheme: Option<&Morpheme>,
) -> Option<Inflection> {
    if let Some(mm) = main_morpheme {
        if morpheme.lexeme == "だ" && !is_continous(mm) {
            return None;
        }
    }

    Some(match morpheme.lexeme {
        "ない" | "ぬ" => Inflection::Negative,
        "ます" | "です" => Inflection::Polite,
        "て" => Inflection::TeForm,
        "だ" | "た" => Inflection::Past,
        "れる" => Inflection::Passive,
        "せる" => Inflection::Causative,
        "られる" => Inflection::CausativePassive,
        "たい" => Inflection::Tai,
        "" => Inflection::Negative,
        _ => return None,
    })
}

fn is_continous(morpheme: &Morpheme) -> bool {
    matches!(morpheme.conjungation.form, ConjungationForm::Continuous)
}

impl<'dict, 'input> InputTextParser<'dict, 'input> {
    /// Creates a new jp text input parser
    pub fn new(
        input: &'input str,
        parser: &'dict Parser,
        in_db: bool,
    ) -> Result<InputTextParser<'dict, 'input>, Error> {
        let input = strip_input(input);

        Ok(InputTextParser {
            morphemes: parser.parse(input),
            original: input,
            in_db,
        })
    }

    /// Tries to understand&parse the input.
    /// Returns a single item with 'surface' set to input
    /// if parsing was not successful or input is a 'valid' word
    pub fn parse(self) -> Option<ParseResult<'dict, 'input>> {
        if self.original.trim().is_empty() {
            return None;
        }

        let inflections = self.get_inflections(
            &self
                .get_inflection_morphemes()
                .iter()
                .map(|i| (*i).to_owned())
                .collect::<Vec<_>>(),
            None,
        );

        if self.is_sentence() && !self.in_db {
            self.parse_sentence()
        } else {
            if !inflections.is_empty() && self.is_word_inflection() && !self.is_sentence() {
                let orig_query = self.original.to_owned();
                let items = self
                    .morphemes
                    .into_iter()
                    // Remove inflection parts
                    .filter(|i| !i.is_inflection())
                    .map(|i| {
                        WordItem::from_morpheme(i, false, inflections.clone(), orig_query.clone())
                    })
                    .collect();

                Some(ParseResult { items })
            } else {
                let lexeme = if self.morphemes.len() == 1 {
                    self.morphemes[0].lexeme
                } else {
                    ""
                };

                // Return blank query
                let items = vec![WordItem {
                    surface: self.original,
                    lexeme,
                    word_class: None,
                    was_in_db: self.in_db,
                    inflections: vec![],
                    start: 0,
                    original_word: String::new(),
                }];
                Some(ParseResult { items })
            }
        }
    }

    fn is_really_inflection(morpheme: &Morpheme, main_morpheme: &Morpheme) -> bool {
        if morpheme.lexeme == "だ" && !is_continous(main_morpheme) {
            false
        } else {
            morpheme.is_inflection()
        }
    }

    /// Returns morphemes with their aux verbs
    fn morpheme_compounds(&self) -> Vec<Vec<Morpheme<'dict, 'input>>> {
        let mut morphemes: Vec<Vec<Morpheme>> = Vec::new();
        let mut curr_morphemes: Vec<Morpheme> = Vec::new();

        for morpheme in self.morphemes.iter() {
            if !curr_morphemes.is_empty() {
                let inflection =
                    Self::is_really_inflection(morpheme, curr_morphemes.last().unwrap());

                if !inflection {
                    morphemes.push(curr_morphemes.clone());
                    curr_morphemes.clear();
                }
            }

            curr_morphemes.push(*morpheme);
        }
        morphemes.push(curr_morphemes);
        morphemes
    }

    // parses a sentence into WordItems
    fn parse_sentence(self) -> Option<ParseResult<'dict, 'input>> {
        let morphemes = self.morpheme_compounds();

        let items = morphemes
            .into_iter()
            .map(|mo| {
                // Split morphemes and aux verbs
                let (morph, aux): (Vec<&Morpheme>, Vec<&Morpheme>) =
                    mo.iter().partition(|i| !i.is_inflection());

                if morph.is_empty() {
                    return aux
                        .iter()
                        .map(|i| {
                            WordItem::from_morpheme(
                                i.to_owned().to_owned(),
                                false,
                                vec![],
                                String::new(),
                            )
                        })
                        .collect();
                }

                let it = morph[0];

                // get inflections for morpheme
                let inflections = self.get_inflections(
                    &aux.iter()
                        .map(|i| i.to_owned().to_owned())
                        .collect::<Vec<Morpheme>>(),
                    Some(it),
                );

                let suffix = aux.into_iter().map(|i| i.surface).join("");
                let original = format!("{}{}", it.surface, suffix);

                vec![WordItem {
                    inflections,
                    was_in_db: false,
                    word_class: Some(it.word_class),
                    lexeme: it.lexeme,
                    surface: it.surface,
                    start: it.start,
                    original_word: original,
                }]
            })
            .flatten()
            .collect::<Vec<_>>();

        Some(ParseResult { items })
    }

    /// Returns true if input text is (very likely) a sentence
    pub fn is_sentence(&self) -> bool {
        self.word_count() > 1 && !self.is_word_inflection()
    }

    /// Returns inflections of the morphemes
    pub fn get_inflections(
        &self,
        morphemes: &[Morpheme],
        main_morpheme: Option<&Morpheme>,
    ) -> Vec<Inflection> {
        if let Some(m) = morphemes.get(0) {
            if morphemes.len() == 1 {
                match m.conjungation.form {
                    ConjungationForm::Imperative
                    | ConjungationForm::Negative
                    | ConjungationForm::Conditional => {
                        return inflection_from_conjungation(&m.conjungation.form)
                            .map(|i| vec![i])
                            .unwrap_or_default()
                    }
                    _ => (),
                }
            }
        }

        let mut inflections: Vec<Inflection> = morphemes
            .iter()
            .filter_map(|i| {
                inflection_from_morpheme(
                    i,
                    main_morpheme.map(Some).unwrap_or_else(|| {
                        self.get_no_inflection_morphemes()
                            .get(0)
                            .map(|i| i.to_owned())
                    }),
                )
            })
            .collect();

        inflections.sort();
        inflections.dedup();
        inflections
    }

    /// Returns true if the input is a single word in form
    /// of an inflection of a word
    pub fn is_word_inflection(&self) -> bool {
        // Check for being only one word
        if self.word_count() > 1 {
            return false;
        }

        if let Some(m) = self.morphemes.get(0) {
            match m.conjungation.form {
                ConjungationForm::Imperative
                | ConjungationForm::Stem
                | ConjungationForm::Negative
                | ConjungationForm::Conditional => return true,
                _ => (),
            }
        }

        are_morphemes_inflection(&self.get_inflection_morphemes())
    }

    fn get_no_inflection_morphemes(&self) -> Vec<&Morpheme> {
        self.morphemes
            .iter()
            .take_while(|morpheme| !morpheme.is_inflection())
            .collect()
    }

    fn get_inflection_morphemes(&self) -> Vec<&Morpheme> {
        self.morphemes
            .iter()
            .rev()
            .take_while(|morpheme| morpheme.is_inflection())
            .collect::<Vec<_>>()
            .into_iter()
            .rev()
            .collect()
    }

    /// Returns the (most likely) amount of words
    pub fn word_count(&self) -> usize {
        self.morphemes
            .iter()
            .filter(|morpheme| morpheme.is_word())
            .count()
    }
}

pub fn are_morphemes_inflection(morphemes: &[&Morpheme]) -> bool {
    match morphemes.len().cmp(&1) {
        Ordering::Equal => morphemes[0].lexeme.eq_any(&INFLECTION_LEXEMES),
        Ordering::Greater => !morphemes
            .iter()
            .map(|i| i.lexeme)
            .any(|i| !i.eq_any(&INFLECTION_LEXEMES)),
        _ => false,
    }
}

fn strip_input(input: &str) -> &str {
    input.trim()
}

pub trait MorphemeExt {
    fn is_inflection(&self) -> bool;
    fn is_word(&self) -> bool;
}

impl<'dict, 'input> MorphemeExt for Morpheme<'dict, 'input> {
    /// Returns true if morpheme is a stand alone word
    fn is_word(&self) -> bool {
        if self.is_inflection() {
            return false;
        }

        match self.word_class {
            WordClass::Adjective(_)
            | WordClass::Adverb
            | WordClass::Pronoun
            | WordClass::Prefix
            | WordClass::PreNoun
            | WordClass::Suffix
            | WordClass::Symbol
            | WordClass::Conjungtion
            | WordClass::Particle(_)
            | WordClass::Noun(_) => return true,
            _ => match self.word_class {
                WordClass::Verb(v) => {
                    if let VerbType::Auxilary(_) = v {
                        return false;
                    } else {
                        return true;
                    }
                }
                _ => (),
            },
        }

        false
    }

    /// Returns true if the morpheme is potentially
    /// representing a part of an inflection
    fn is_inflection(&self) -> bool {
        match self.word_class {
            WordClass::Verb(VerbType::Auxilary(_)) => true,
            WordClass::Particle(p) => {
                matches!(p, ParticleType::SentenceEnding | ParticleType::Conjungtion)
            }
            _ => false,
        }
    }
}

trait StrExt {
    fn ends_any(&self, cont: &[&str]) -> bool;
    fn contains_any(&self, cont: &[&str]) -> bool;
    fn eq_any(&self, cont: &[&str]) -> bool;
}

impl StrExt for str {
    fn ends_any(&self, cont: &[&str]) -> bool {
        for c in cont {
            if self.ends_with(c) {
                return true;
            }
        }

        false
    }

    fn contains_any(&self, cont: &[&str]) -> bool {
        for c in cont {
            if self.contains(c) {
                return true;
            }
        }

        false
    }

    fn eq_any(&self, cont: &[&str]) -> bool {
        for c in cont {
            if self.eq(*c) {
                return true;
            }
        }

        false
    }
}
