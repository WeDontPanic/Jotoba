use super::part_of_speech::{self, IrregularVerb, PartOfSpeech};

use super::Word;

use jp_inflections::{Verb, VerbType, WordForm};
use serde::{Deserialize, Serialize};

/// A single Inflection
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize)]
pub enum Inflection {
    Negative,
    Polite,
    Present,
    Past,
    TeForm,
    Potential,
    Passive,
    Causative,
    CausativePassive,
    PotentialOrPassive,
    Imperative,
    Tai,
    TeIru,
    TeAru,
    TeMiru,
    TeShimau,
    Chau,
    TeOku,
    Toku,
    Tara,
    Tari,
    Ba,
}

#[cfg(feature = "jotoba_intern")]
impl localization::traits::Translatable for Inflection {
    fn get_id(&self) -> &'static str {
        match self {
            Inflection::Negative => "Negative",
            Inflection::Polite => "Polite",
            Inflection::Present => "Present",
            Inflection::Past => "Past",
            Inflection::TeForm => "TeForm",
            Inflection::Potential => "Potential",
            Inflection::Passive => "Passive",
            Inflection::Causative => "Causative",
            Inflection::CausativePassive => "CausativePassive",
            Inflection::PotentialOrPassive => "PotentialOrPassive",
            Inflection::Imperative => "Imperative",
            Inflection::Tai => "Tai",
            Inflection::TeIru => "TeIru",
            Inflection::TeAru => "TeAru",
            Inflection::TeMiru => "TeMiru",
            Inflection::TeShimau => "TeShimau",
            Inflection::TeOku => "TeOku",
            Inflection::Chau => "Chau",
            Inflection::Toku => "Toku",
            Inflection::Tara => "Tara",
            Inflection::Tari => "Tari",
            Inflection::Ba => "Ba",
        }
    }

    fn gettext<'a>(
        &self,
        dict: &'a localization::TranslationDict,
        language: Option<localization::language::Language>,
    ) -> &'a str {
        self.pgettext(dict, "inflection", language)
    }
}

/// A set of different inflections which will be displayed for vebs
#[derive(Serialize, Deserialize)]
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

#[derive(Serialize, Deserialize)]
pub struct InflectionPair {
    #[serde(rename = "p")]
    pub positive: String,
    #[serde(rename = "n")]
    pub negative: String,
}

/// Returns the inflections of `word` if its a verb
pub(super) fn of_word(word: &Word) -> Option<Inflections> {
    let verb = get_jp_verb(word)?;
    let build = || -> Result<Inflections, jp_inflections::error::Error> {
        let is_exception = word.reading.kanji
            .as_ref()
            .map(|kanji| kanji.reading == "為る" || kanji.reading == "来る")
            .unwrap_or(false);

        return Ok(Inflections {
            present: InflectionPair {
                positive: verb.dictionary(WordForm::Short)?.try_kanji(is_exception),
                negative: verb.negative(WordForm::Short)?.try_kanji(is_exception),
            },
            present_polite: InflectionPair {
                positive: verb.dictionary(WordForm::Long)?.try_kanji(is_exception),
                negative: verb.negative(WordForm::Long)?.try_kanji(is_exception),
            },

            past: InflectionPair {
                positive: verb.past(WordForm::Short)?.try_kanji(is_exception),
                negative: verb.negative_past(WordForm::Short)?.try_kanji(is_exception),
            },
            past_polite: InflectionPair {
                positive: verb.past(WordForm::Long)?.try_kanji(is_exception),
                negative: verb.negative_past(WordForm::Long)?.try_kanji(is_exception),
            },
            te_form: InflectionPair {
                positive: verb.te_form()?.try_kanji(is_exception),
                negative: verb.negative_te_form()?.try_kanji(is_exception),
            },
            potential: InflectionPair {
                positive: verb.potential(WordForm::Short)?.try_kanji(is_exception),
                negative: verb.negative_potential(WordForm::Short)?.try_kanji(is_exception),
            },
            passive: InflectionPair {
                positive: verb.passive()?.try_kanji(is_exception),
                negative: verb.negative_passive()?.try_kanji(is_exception),
            },
            causative: InflectionPair {
                positive: verb.causative()?.try_kanji(is_exception),
                negative: verb.negative_causative()?.try_kanji(is_exception),
            },
            causative_passive: InflectionPair {
                positive: verb.causative_passive()?.try_kanji(is_exception),
                negative: verb.negative_causative_passive()?.try_kanji(is_exception),
            },
            imperative: InflectionPair {
                positive: verb.imperative()?.try_kanji(is_exception),
                negative: verb.imperative_negative()?.try_kanji(is_exception),
            },
        });
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
