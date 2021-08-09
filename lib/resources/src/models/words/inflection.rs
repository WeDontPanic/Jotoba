use super::Word;

use jp_inflections::{Verb, VerbType, WordForm};

/// A set of different inflections which will be displayed for vebs
pub struct Inflections {
    pub present: InflectionPair,
    pub present_polite: InflectionPair,

    pub past: InflectionPair,
    pub past_polite: InflectionPair,

    pub te_form: InflectionPair,

    pub potential: InflectionPair,
    pub passive: InflectionPair,
    pub causative: InflectionPair,
}

pub struct InflectionPair {
    pub positive: String,
    pub negative: String,
}

pub(super) fn of_word(word: &Word) -> Option<Inflections> {
    None
}

/// Returns a jp_inflections::Verb if [`self`] is a verb
fn get_jp_verb(word: &Word) -> Option<Verb> {
    let verb_type = if word.get_pos().any(|i| i.is_ichidan()) {
        VerbType::Ichidan
    } else if word.get_pos().any(|i| i.is_godan()) {
        VerbType::Godan
    } else {
        return None;
    };

    let verb = Verb::new(
        jp_inflections::Word::new(
            &word.reading.kana.reading,
            word.reading.kanji.as_ref().map(|i| &i.reading),
        ),
        verb_type,
    );

    // Check if [`verb`] really is a valid verb in dictionary form
    verb.word.is_verb().then(|| verb)
}
