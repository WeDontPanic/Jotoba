use super::FromMorphemes;
use crate::grammar::{rule::Rule, rule_set::RuleSet, Analyzer};
use crate::sentence::SentenceAnalyzer;
use igo_unidic::Morpheme;
use once_cell::sync::Lazy;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Inflection {
    Negative,
    Polite,
    Present,
    Past,
    TeForm,
    Potential,
    Passive,
    Causative,
    PotentialOrPassive,
    Imperative,
    Tai,
    TeIru,
    TeAru,
    TeMiru,
    Tara,
}

impl<'b> FromMorphemes<'static, 'b> for Inflection {
    /// Parses an inflection from given morpheme(s)
    fn from(parts: Vec<igo_unidic::Morpheme<'static, 'b>>, _pos: usize) -> Option<Self> {
        let lexemes = parts.iter().map(|i| i.lexeme).collect::<Vec<_>>();

        if lexemes.is_empty() {
            None
        } else if lexemes.len() == 1 {
            if parts[0].surface == "たら"{
                return Some(Self::Tara);
            }

            Some(match lexemes[0] {
                "ない" | "ぬ" => Inflection::Negative,
                "ます" => Inflection::Polite,
                "て" => Inflection::TeForm,
                "だ" | "た" => Inflection::Past,
                "れる" => Inflection::Passive,
                "せる" | "させる" => Inflection::Causative,
                "られる" => Inflection::PotentialOrPassive,
                "たい" => Inflection::Tai,
                "てる" => Inflection::TeIru,
                //"てる" => Inflection::TeIru,
                _ => return None,
            })
        } else {
            Some(match lexemes.as_slice() {
                &["て", "いる"] => Inflection::TeIru,
                &["て", "ある"] => Inflection::TeAru,
                &["て", "みる"] => Inflection::TeMiru,
                &["さ", "せる"] => Inflection::Causative,
                _ => return None,
            })
        }
    }
}

pub(crate) fn parse_inflections(morph: &[Morpheme<'static, '_>]) -> Vec<Inflection> {
    SentenceAnalyzer::new(&INFLECTION_RULES, morph.to_vec()).analyze::<Inflection>()
}

static INFLECTION_RULES: Lazy<Analyzer> = Lazy::new(|| Analyzer::new(get_rules()));

/// Returns a set of rules for japanese text analyzing
fn get_rules() -> RuleSet {
    let mut rules = Vec::with_capacity(5);

    rules.push(Rule::new("いる", &[]));
    rules.push(Rule::new("ある", &[]));
    rules.push(Rule::new("てみる", &[]));

    rules.push(Rule::new("て", &["いる", "ある", "てみる"]));

    RuleSet::new(&rules)
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
            Inflection::PotentialOrPassive => "PotentialOrPassive",
            Inflection::Imperative => "Imperative",
            Inflection::Tai => "Tai",
            Inflection::TeIru => "TeIru",
            Inflection::TeAru => "TeAru",
            Inflection::TeMiru => "TeMiru",
            Inflection::Tara=> "Tara",
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
