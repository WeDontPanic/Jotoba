use super::FromMorphemes;
use crate::grammar::{rule::Rule, rule_set::RuleSet, Analyzer};
use crate::sentence::SentenceAnalyzer;
use igo_unidic::Morpheme;
use once_cell::sync::Lazy;
use types::jotoba::words::inflection::Inflection;

/*
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
}
*/

impl<'b> FromMorphemes<'static, 'b> for Inflection {
    /// Parses an inflection from given morpheme(s)
    fn from(parts: Vec<igo_unidic::Morpheme<'static, 'b>>, _pos: usize) -> Option<Self> {
        let lexemes = parts.iter().map(|i| i.lexeme).collect::<Vec<_>>();

        if lexemes.is_empty() {
            None
        } else if lexemes.len() == 1 {
            if parts[0].surface == "たら" {
                return Some(Self::Tara);
            }

            Some(match lexemes[0] {
                "ない" | "ぬ" => Inflection::Negative,
                "ます" => Inflection::Polite,
                "て" | "で" => Inflection::TeForm,
                "だ" | "た" => Inflection::Past,
                "れる" => Inflection::Passive,
                "せる" | "させる" => Inflection::Causative,
                "られる" => Inflection::PotentialOrPassive,
                "たい" => Inflection::Tai,
                "たり" | "だり" => Inflection::Tari,
                "てる" | "でる" => Inflection::TeIru,
                "とく" | "どく" => Inflection::Toku,
                "ちゃう" | "じゃう" => Inflection::Chau,
                "ば" => Inflection::Ba,
                _ => return None,
            })
        } else {
            Some(match lexemes.as_slice() {
                &["て", "いる"] | &["で", "いる"] => Inflection::TeIru,
                &["て", "ある"] | &["で", "ある"] => Inflection::TeAru,
                &["て", "みる"] | &["で", "みる"] => Inflection::TeMiru,
                &["て", "しまう"] | &["で", "しまう"] => Inflection::TeShimau,
                &["て", "おく"] | &["で", "おく"] => Inflection::TeOku,
                &["さ", "せる"] => Inflection::Causative,
                // Fake する; The tokenizer tokenizes the さ of される as a form of する
                &["する", "れる"] => Inflection::CausativePassive,
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
    let mut rules = Vec::with_capacity(7);

    rules.push(Rule::new("いる", &[]));
    rules.push(Rule::new("ある", &[]));
    rules.push(Rule::new("てみる", &[]));
    rules.push(Rule::new("しまう", &[]));
    rules.push(Rule::new("おく", &[]));
    rules.push(Rule::new("れる", &[]));

    rules.push(Rule::new(
        "て",
        &["いる", "ある", "てみる", "しまう", "おく"],
    ));
    rules.push(Rule::new("さ", &["れる"]));

    RuleSet::new(&rules)
}
