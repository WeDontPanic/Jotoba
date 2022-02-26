use crate::grammar::{rule::Rule, rule_set::RuleSet, Analyzer};
use once_cell::sync::Lazy;

static RULES: Lazy<Analyzer> = Lazy::new(|| Analyzer::new(get_rules()));

/// Returns a grammar analyzer with a japanese inflection ruleset
pub(crate) fn get_grammar_analyzer() -> &'static Analyzer {
    &RULES
}

/// Returns a set of rules for japanese text analyzing
fn get_rules() -> RuleSet {
    // Often used dest rules
    let end = &[];
    let te_ending = &[
        "て",
        "てる",
        "ます",
        "しまう",
        "ない",
        "た",
        "てみる",
        "いる",
        "ある",
    ];
    let ru_ending = &["て", "てる", "ます", "しまう", "ない", "た", "ちゃう"];
    // \ Often used dest rules

    // い rule
    let rule_た = Rule::new("た", end);
    let rule_ない = Rule::new("ない", &["て", "た"]);
    let rule_たい = Rule::new("たい", &["ない", "た"]);

    // じゃない
    let rule_じゃ = Rule::new("じゃ", &["ない"]);

    // て
    let rule_て = Rule::new("て", te_ending);
    let rule_てみて = Rule::new("てみる", ru_ending);
    let rule_てる = Rule::new("てる", ru_ending);

    // いる/ある
    let rule_いる = Rule::new("いる", ru_ending);
    let rule_ある = Rule::new("ある", ru_ending);

    // Masu
    let rule_ます = Rule::new("ます", &["た", "ん"]);
    let rule_ん = Rule::new("ん", &["です"]);
    let rule_です = Rule::new("です", &["た"]);

    // passive / 可能形
    let rule_られる = Rule::new("られる", ru_ending);
    let rule_れる = Rule::new("れる", ru_ending);

    // ちゃう / しまう
    let rule_ちゃう = Rule::new("ちゃう", ru_ending);
    let rule_しまう = Rule::new("しまう", ru_ending);

    // Generation/Root
    let rule_verb = Rule::new(
        "V",
        &[
            "た",
            "ない",
            "たい",
            "て",
            "てる",
            "てみる",
            "いる",
            "ある",
            "ます",
            "られる",
            "れる",
            "ちゃう",
            "しまう",
        ],
    );

    let rule_adjective = Rule::new("AD", &["ない", "た", "て"]);
    let rule_number = Rule::new("NR", &["NR"]);

    // generate ruleset
    RuleSet::new(&[
        rule_ない,
        rule_じゃ,
        rule_たい,
        rule_た,
        rule_ます,
        rule_て,
        rule_てみて,
        rule_てる,
        rule_いる,
        rule_ある,
        rule_られる,
        rule_れる,
        rule_ん,
        rule_です,
        rule_ちゃう,
        rule_しまう,
        rule_verb,
        rule_adjective,
        rule_number,
    ])
}
