use super::part_of_speech::{self, IrregularVerb, PartOfSpeech};

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

    pub causative_passive: InflectionPair,
    pub imperative: InflectionPair,
}

pub struct InflectionPair {
    pub positive: String,
    pub negative: String,
}

/// Returns the inflections of `word` if its a verb
pub(super) fn of_word(word: &Word) -> Option<Inflections> {
    let verb = get_jp_verb(word)?;
    let build = || -> Result<Inflections, jp_inflections::error::Error> {
        Ok(Inflections {
            present: InflectionPair {
                positive: verb.dictionary(WordForm::Short)?.get_reading(),
                negative: verb.negative(WordForm::Short)?.get_reading(),
            },
            present_polite: InflectionPair {
                positive: verb.dictionary(WordForm::Long)?.get_reading(),
                negative: verb.negative(WordForm::Long)?.get_reading(),
            },

            past: InflectionPair {
                positive: verb.past(WordForm::Short)?.get_reading(),
                negative: verb.negative_past(WordForm::Short)?.get_reading(),
            },
            past_polite: InflectionPair {
                positive: verb.past(WordForm::Long)?.get_reading(),
                negative: verb.negative_past(WordForm::Long)?.get_reading(),
            },
            te_form: InflectionPair {
                positive: verb.te_form()?.get_reading(),
                negative: verb.negative_te_form()?.get_reading(),
            },
            potential: InflectionPair {
                positive: verb.potential(WordForm::Short)?.get_reading(),
                negative: verb.negative_potential(WordForm::Short)?.get_reading(),
            },
            passive: InflectionPair {
                positive: verb.passive()?.get_reading(),
                negative: verb.negative_passive()?.get_reading(),
            },
            causative: InflectionPair {
                positive: verb.causative()?.get_reading(),
                negative: verb.negative_causative()?.get_reading(),
            },
            causative_passive: InflectionPair {
                positive: verb.causative_passive()?.get_reading(),
                negative: verb.negative_causative_passive()?.get_reading(),
            },
            imperative: InflectionPair {
                positive: verb.imperative()?.get_reading(),
                negative: verb.imperative_negative()?.get_reading(),
            },
        })
    }()
    .ok()?;

    Some(build)
}

/// Returns a jp_inflections::Verb if [`self`] is a verb
fn get_jp_verb(word: &Word) -> Option<Verb> {
    let is_suru = word.get_pos().any(|i| match i {
        PartOfSpeech::Verb(v) => match v {
            part_of_speech::VerbType::Irregular(i) => match i {
                IrregularVerb::Suru => true,
                _ => false,
            },
            _ => false,
        },
        _ => false,
    });

    let verb_type = if word.get_pos().any(|i| i.is_ichidan()) {
        VerbType::Ichidan
    } else if word.get_pos().any(|i| i.is_godan()) {
        VerbType::Godan
    } else if is_suru {
        VerbType::Exception
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
