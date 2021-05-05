use crate::{error::Error, japanese::Inflection, DbPool};
use diesel::{dsl::exists, prelude::*};
use igo_unidic::{ConjungationForm, Morpheme, Parser, ParticleType, VerbType, WordClass};
use tokio_diesel::AsyncRunQueryDsl;

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

async fn db_contains_word(db: &DbPool, word: &str) -> Result<bool, Error> {
    use crate::schema::dict::dsl::*;
    Ok(diesel::select(exists(dict.filter(reading.eq(word))))
        .get_result_async(db)
        .await?)
}

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
}

impl<'dict, 'input> WordItem<'dict, 'input> {
    fn from_morpheme(
        m: Morpheme<'dict, 'input>,
        in_db: bool,
        inflections: Vec<Inflection>,
    ) -> Self {
        WordItem {
            word_class: Some(m.word_class),
            lexeme: m.lexeme,
            surface: m.surface,
            was_in_db: in_db,
            inflections,
        }
    }

    pub fn get_lexeme(&self) -> &str {
        if self.lexeme.is_empty() {
            self.surface
        } else {
            self.lexeme
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ParseResult<'dict, 'input> {
    pub items: Vec<WordItem<'dict, 'input>>,
}

impl Inflection {
    fn from_conjungation(conj: &ConjungationForm) -> Option<Self> {
        Some(match conj {
            ConjungationForm::Plain => Inflection::Present,
            ConjungationForm::Imperative => Inflection::Imperative,
            ConjungationForm::Negative => Inflection::Negative,
            ConjungationForm::Conditional => Inflection::Potential,
            _ => return None,
        })
    }

    fn from_morpheme(morpheme: &Morpheme, main_morpheme: Option<&Morpheme>) -> Option<Self> {
        if let Some(mm) = main_morpheme {
            if morpheme.lexeme == "だ" && !is_continous(mm) {
                return None;
            }
        }

        Some(match morpheme.lexeme {
            "ない" | "ぬ" => Self::Negative,
            "ます" | "です" => Self::Polite,
            "て" => Self::TeForm,
            "だ" | "た" => Self::Past,
            "れる" => Self::Passive,
            "せる" => Self::Causative,
            "られる" => Self::CausativePassive,
            "たい" => Self::Tai,
            "" => Self::Negative,
            _ => return None,
        })
    }
}

fn is_continous(morpheme: &Morpheme) -> bool {
    match morpheme.conjungation.form {
        ConjungationForm::Continuous => true,
        _ => false,
    }
}

impl<'dict, 'input> InputTextParser<'dict, 'input> {
    /// Creates a new jp text input parser
    pub async fn new(
        db: &DbPool,
        input: &'input str,
        parser: &'dict Parser,
    ) -> Result<InputTextParser<'dict, 'input>, Error> {
        let input = strip_input(input);
        let in_db = db_contains_word(db, &input).await?;

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
                .map(|i| i.clone().to_owned())
                .collect::<Vec<_>>(),
            None,
        );

        println!("sentence: {}", self.is_sentence());
        println!("in db: {}", self.in_db);
        println!("inflecions: {:#?}", inflections);
        println!("word inflection: {:}", self.is_word_inflection());

        if self.is_sentence() && !self.in_db {
            self.parse_sentence()
        } else {
            if !inflections.is_empty() && self.is_word_inflection() && !self.is_sentence() {
                let items = self
                    .morphemes
                    .into_iter()
                    // Remove inflection parts
                    .filter(|i| !i.is_inflection())
                    .map(|i| WordItem::from_morpheme(i, false, inflections.clone()))
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

            curr_morphemes.push(morpheme.clone());
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
                        .map(|i| WordItem::from_morpheme(i.to_owned().to_owned(), false, vec![]))
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

                vec![WordItem {
                    inflections,
                    was_in_db: false,
                    word_class: Some(it.word_class),
                    lexeme: it.lexeme,
                    surface: it.surface,
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
                        return Inflection::from_conjungation(&m.conjungation.form)
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
                Inflection::from_morpheme(
                    &i,
                    main_morpheme.map(|i| Some(i)).unwrap_or_else(|| {
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
    if morphemes.len() == 1 {
        morphemes[0].lexeme.eq_any(&INFLECTION_LEXEMES)
    } else if morphemes.len() > 1 {
        !morphemes
            .iter()
            .map(|i| i.lexeme)
            .any(|i| !i.eq_any(&INFLECTION_LEXEMES))
    } else {
        false
    }
}

fn strip_input(input: &str) -> &str {
    let input = input.trim();
    input
}

pub trait MorphemeExt {
    fn is_inflection(&self) -> bool;
    fn is_word(&self) -> bool;
}

impl<'dict, 'input> MorphemeExt for Morpheme<'dict, 'input> {
    /// Returns true if morpheme is a stand alone word
    fn is_word(&self) -> bool {
        match self.word_class {
            WordClass::Adjective(_)
            | WordClass::Adverb
            | WordClass::Pronoun
            | WordClass::Prefix
            | WordClass::PreNoun
            | WordClass::Suffix
            | WordClass::Noun(_) => return true,
            _ => match self.word_class {
                WordClass::Verb(v) => match v {
                    VerbType::Auxilary(_) => return false,
                    _ => return true,
                },
                _ => (),
            },
        }

        false
    }

    /// Returns true if the morpheme is potentially
    /// representing a part of an inflection
    fn is_inflection(&self) -> bool {
        match self.word_class {
            WordClass::Verb(v) => match v {
                VerbType::Auxilary(_) => true,
                _ => false,
            },
            WordClass::Particle(p) => match p {
                ParticleType::SentenceEnding | ParticleType::Conjungtion => true,
                _ => false,
            },
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
