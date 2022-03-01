use super::{
    inflection::{self, Inflection},
    owned_morpheme::OwnedMorpheme,
    FromMorphemes,
};
use igo_unidic::{Morpheme, WordClass};
use japanese::JapaneseExt;

/// A single word within a sentence. This already contains all inflection parts
#[derive(Debug, Clone, PartialEq)]
pub struct Part {
    /// All morphemes building the (inflected) word
    morphemes: Vec<OwnedMorpheme<'static>>,
    inflections: Vec<Inflection>,
    pos: usize,
    furigana: Option<String>,
}

impl Part {
    /// Creates a new sentence part. Automatically parses additional morphemes to inflections
    pub fn new(morphemes: Vec<Morpheme<'static, '_>>, pos: usize) -> Option<Self> {
        if morphemes.len() == 0 {
            return None;
        }

        // parse inflections
        let inflections = inflection::parse_inflections(&morphemes[1..]);

        // get them owned
        let morphemes = morphemes.into_iter().map(|i| i.into()).collect::<Vec<_>>();

        Some(Self {
            furigana: None,
            inflections,
            pos,
            morphemes,
        })
    }

    /// Returns `true` if the part has at least one inflection
    pub fn has_inflections(&self) -> bool {
        !self.inflections().is_empty()
    }

    /// Get a reference to the parts morphemes.
    pub fn morphemes(&self) -> &[OwnedMorpheme] {
        &self.morphemes
    }

    /// Get a reference to the word's inflections.
    pub fn inflections(&self) -> &[Inflection] {
        &self.inflections
    }

    /// Returns the full surface of the part. If it has inflections, this surface represents the
    /// word written with all inflections. If there are no inflections, this method returns the
    /// same as `get_normalized()`
    pub fn get_inflected(&self) -> String {
        self.morphemes
            .iter()
            .map(|i| i.surface.as_str())
            .collect::<String>()
    }

    /// Returns the normalized form of the word. All inflections are removed and the dictionary
    /// form of the word is returned
    pub fn get_normalized(&self) -> String {
        self.get_main_morpheme().lexeme.to_string()
    }

    /// Get the part's pos.
    pub fn pos(&self) -> usize {
        self.pos
    }

    /// Sets the furigana
    pub fn set_furigana<F>(&mut self, add_fn: F)
    where
        F: Fn(&str) -> Option<String>,
    {
        let mut out = String::new();
        let mut has_furigana = false;

        for morpheme in &self.morphemes {
            if !morpheme.surface.has_kanji() {
                out.push_str(&morpheme.surface);
            } else if let Some(furi) = add_fn(morpheme.lexeme) {
                // check if `furi` really contains furigana. If this is not the case but
                // `has_furigana` is true, the text will be rendered weird
                if furi.contains('|') {
                    has_furigana = true;
                }
                // TODO: fix 食ってないこと
                out.push_str(&furi);
            } else {
                out.push_str(&morpheme.surface);
            }
        }

        if has_furigana {
            self.furigana = Some(out);
        }
    }

    /// Returns furigana of the word
    pub fn furigana(&self) -> Option<&str> {
        self.furigana.as_deref()
    }

    /// returns msgid for the current word_class or None if no word_class is set
    pub fn word_class(&self) -> Option<&'static str> {
        Some(match self.get_main_morpheme().word_class {
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

    pub fn word_class_raw(&self) -> &WordClass<'_> {
        &self.get_main_morpheme().word_class
    }

    /// Gets wordclass in lowercase
    pub fn word_class_lower(&self) -> Option<String> {
        self.word_class().map(|i| i.to_lowercase())
    }

    /// Returns the morpheme containing the actual 'word' without any inflections
    fn get_main_morpheme(&self) -> &OwnedMorpheme {
        &self.morphemes[0]
    }
}

impl<'b> FromMorphemes<'static, 'b> for Part {
    #[inline]
    fn from(parts: Vec<Morpheme<'static, 'b>>, pos: usize) -> Option<Self> {
        Self::new(parts, pos)
    }
}
