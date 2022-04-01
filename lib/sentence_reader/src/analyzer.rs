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
        "おく",
    ];
    let ru_ending = &[
        "て",
        "てる",
        "ます",
        "しまう",
        "ない",
        "た",
        "たり",
        "ちゃう",
        "とく",
        "たい",
        "られる",
        "れる",
        "ば",
    ];
    // \ Often used dest rules
    //

    let mut rules = Vec::with_capacity(20);

    // い rule
    rules.push(Rule::new("た", end));
    rules.push(Rule::new("たり", end));
    rules.push(Rule::new("ない", &["て", "た"]));
    rules.push(Rule::new("たい", &["て", "ない", "た"]));

    // じゃない
    rules.push(Rule::new("じゃ", &["ない"]));

    // て
    rules.push(Rule::new("て", te_ending));
    rules.push(Rule::new("てみる", ru_ending));
    rules.push(Rule::new("しまう", ru_ending));
    rules.push(Rule::new("おく", ru_ending));
    rules.push(Rule::new("てる", ru_ending));

    // いる/ある
    rules.push(Rule::new("いる", ru_ending));
    rules.push(Rule::new("ある", ru_ending));

    // Masu
    rules.push(Rule::new("ます", &["た", "ん"]));
    rules.push(Rule::new("ん", &["です"]));
    rules.push(Rule::new("です", &["た"]));

    // passive / 可能形
    rules.push(Rule::new("られる", ru_ending));
    rules.push(Rule::new("れる", ru_ending));

    // ちゃう / しまう
    rules.push(Rule::new("ちゃう", ru_ending));
    rules.push(Rule::new("しまう", ru_ending));

    // とく
    rules.push(Rule::new("とく", ru_ending));

    // ば conditional
    rules.push(Rule::new("ば", end));

    // される causative
    rules.push(Rule::new("さ", &["せる", "れる"]));
    rules.push(Rule::new("せる", ru_ending));
    rules.push(Rule::new("させる", ru_ending));

    // Exceptions
    rules.push(Rule::new("いただき", &["ます"]));

    // Generation/Root
    rules.push(Rule::new(
        "V",
        &[
            "た",
            "たり",
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
            "とく",
            "ば",
            "せる",
            "させる",
            // the さ of される
            "さ",
        ],
    ));

    rules.push(Rule::new("AD", &["ない", "た", "て"]));
    rules.push(Rule::new("NR", &["NR"]));

    // generate ruleset
    RuleSet::new(&rules)
}
