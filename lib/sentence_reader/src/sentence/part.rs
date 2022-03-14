use super::{inflection, owned_morpheme::OwnedMorpheme, FromMorphemes};
use igo_unidic::{Morpheme, WordClass};
use japanese::{CharType, JapaneseExt};
use types::jotoba::words::inflection::Inflection;

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
            } else if let Some(furi) = add_fn(morpheme.reading()) {
                // check if `furi` really contains furigana. If this is not the case but
                // `has_furigana` is true, the text will be rendered weird
                if !furi.contains('|') {
                    out.push_str(&furi);
                } else {
                    has_furigana = true;
                    out.push_str(&merge_furigana(&morpheme.surface, &furi));
                }
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

    /// Gets the main lexeme. Falls back on surface if lexeme is empty
    fn main_lexeme(&self) -> &str {
        self.get_main_morpheme().reading()
    }
}

impl<'b> FromMorphemes<'static, 'b> for Part {
    #[inline]
    fn from(parts: Vec<Morpheme<'static, 'b>>, pos: usize) -> Option<Self> {
        Self::new(parts, pos)
    }
}

/// Merges a reading with its given furigana. This is required for cases where `furi` does not
/// represent he same kana reading as `src`.
///
/// Example:
/// src: "行った" furi: "[行|い]く" => [行|い]った
fn merge_furigana(src: &str, furi: &str) -> String {
    let mut furi_out = String::with_capacity(furi.len());
    let mut kana_paths = japanese::all_words_with_ct(src, CharType::Kana).into_iter();

    for furi_part in japanese::furigana::from_str(furi) {
        if furi_part.kanji.is_none() {
            if let Some(next) = kana_paths.next() {
                furi_out.push_str(&next);
            }
            continue;
        }

        furi_out.push_str(&furi_part.encoded());
    }

    furi_out
}

impl Into<types::api::app::responses::words::SentencePart> for Part {
    #[inline]
    fn into(self) -> types::api::app::responses::words::SentencePart {
        let furigana = self.furigana().map(|i| i.to_string());
        let position = self.pos();
        let inflected = self.get_inflected();
        let word_class = self.word_class();
        types::api::app::responses::words::SentencePart::new(
            furigana, position, inflected, word_class,
        )
    }
}
