use super::{inflection::Inflection, owned_morpheme::OwnedMorpheme};
use igo_unidic::{Morpheme, WordClass};

/// A single word within a sentence. This already contains all inflection parts
#[derive(Debug)]
pub struct Part {
    /// All morphemes building the (inflected) word
    morphemes: Vec<OwnedMorpheme<'static>>,
    inflections: Vec<Inflection>,
    pos: usize,
    furigana: String,
}

impl Part {
    #[inline]
    pub fn new(morphemes: Vec<Morpheme<'static, '_>>, pos: usize) -> Option<Self> {
        if morphemes.len() == 0 {
            return None;
        }

        let mut inflections = morphemes
            .iter()
            // skip main morpheme at position 0
            .skip(1)
            .filter_map(|i| morph_to_inflection(i))
            .collect::<Vec<_>>();
        inflections.sort();
        //inflections.dedup();

        let morphemes = morphemes.into_iter().map(|i| i.into()).collect::<Vec<_>>();

        let furigana = String::new();

        Some(Self {
            morphemes,
            inflections,
            pos,
            furigana,
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

    /// Returns the morpheme containing the actual 'word' without any inflections
    fn get_main_morpheme(&self) -> &OwnedMorpheme {
        &self.morphemes[0]
    }

    /// Returns furigana of the word
    fn get_furigana(&self) -> &str {
        &self.furigana
    }

    /// returns msgid for the current word_class or None if no word_class is set
    fn word_class_to_str(&self) -> Option<&'static str> {
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
}

fn morph_to_inflection(morph: &Morpheme) -> Option<Inflection> {
    /*
    if let Some(infl) = inflection_from_conjungation(&morph.conjungation.form){
        return Some(infl);
    }
    */
    Some(match morph.lexeme {
        "ない" | "ぬ" => Inflection::Negative,
        "ます" | "です" => Inflection::Polite,
        "て" => Inflection::TeForm,
        "だ" | "た" => Inflection::Past,
        "れる" => Inflection::Passive,
        "せる" | "させる" => Inflection::Causative,
        "られる" => Inflection::PotentialOrPassive,
        "たい" => Inflection::Tai,
        _ => return None,
    })
}

/*
fn inflection_from_conjungation(conj: &ConjungationForm) -> Option<Inflection> {
    Some(match conj {
        ConjungationForm::Plain => Inflection::Present,
        ConjungationForm::Imperative => Inflection::Imperative,
        ConjungationForm::Negative => Inflection::Negative,
        ConjungationForm::Conditional => Inflection::Potential,
        _ => return None,
    })
}
*/
