#![allow(dead_code)]

use self::{
    rule::{Rule, ToRule},
    rule_set::RuleSet,
};

pub mod rule;
pub mod rule_set;

/// A Grammar analyzer
#[derive(Clone)]
pub struct Analyzer {
    rules: RuleSet,
}

impl Analyzer {
    /// Creates a new Grammar analyzer
    pub fn new(rules: RuleSet) -> Self {
        Self { rules }
    }

    /// Checks if `inp` can be built with the given ruleset. Returns the index of the last rule
    /// that was matching. In other words if the return value is equal to `inp.len()`, all input
    /// rules were matching
    pub fn check<T: ToRule>(&self, inp: &[T]) -> usize {
        if inp.is_empty() {
            return 0;
        }

        let mut pos = 0;

        let mut last_rule = match self.resolve_to_rule(&inp[0]) {
            Some(r) => r,
            None => return pos,
        };

        pos += 1;

        for part in &inp[pos..] {
            let rule = match self.resolve_to_rule(part) {
                Some(r) => r,
                None => return pos,
            };

            if !last_rule.has_dst(rule.name()) {
                return pos;
            }

            last_rule = rule;
            pos += 1;
        }

        pos
    }

    /// Returns `true` if the analyzer has a given rule
    #[inline]
    pub fn has_rule(&self, rule: &str) -> bool {
        self.rules.get_rule(rule).is_some()
    }

    /// Checks if a series of Rules can be built with the current set of Rules
    #[inline]
    pub fn check_full<T: ToRule>(&self, inp: &[T]) -> bool {
        self.check(inp) == inp.len()
    }

    /// resolves a rule from `ToRule` to `&Rule`
    #[inline]
    fn resolve_to_rule<T: ToRule>(&self, tr: T) -> Option<&Rule> {
        tr.to_rule().and_then(|i| self.rules.get_rule(i))
    }

    /// Get a reference to the analyzer's rules.
    #[inline]
    pub fn rules(&self) -> &RuleSet {
        &self.rules
    }
}
