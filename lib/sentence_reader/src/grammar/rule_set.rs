use super::rule::Rule;
use std::{collections::HashMap, fmt::Debug};

pub const ALL_WILDCARD: &str = "*";

#[derive(Clone)]
pub struct RuleSet {
    rules: HashMap<&'static str, Rule>,
}

impl RuleSet {
    /// Creates a new set of rules
    pub fn new(rules: &[Rule]) -> Self {
        let rules = rules
            .iter()
            .map(|i| (i.name(), *i))
            .collect::<HashMap<_, _>>();
        Self { rules }
    }

    /// Adds a Rule to the RuleSet
    pub fn add(&mut self, rule: Rule) -> bool {
        if self.has_rule(rule.name()) {
            return false;
        }

        // add dummy rule to allow any dst rule
        if rule.has_dst(ALL_WILDCARD) {
            self.add_all_wildcard();
        }

        self.rules.insert(rule.name(), rule);
        true
    }

    /// Returns `true` if ruleSet has a rule with `name`
    pub fn has_rule(&self, name: &str) -> bool {
        self.rules.contains_key(name)
    }

    /// Returns `true` if the RuleSet is complete
    pub fn check(&self) -> bool {
        // check that all used dst rules are reachable
        for (_, rule) in self.rules.iter() {
            for rhs in rule.rhs() {
                if *rhs == ALL_WILDCARD {
                    continue;
                }
                if !self.rules.contains_key(rhs) {
                    return false;
                }
            }
        }

        true
    }

    /// Returns a rule with `name` or None when no such rule exists in RuleSet
    #[inline]
    pub fn get_rule(&self, name: &str) -> Option<&Rule> {
        self.rules.get(name)
    }

    fn add_all_wildcard(&mut self) {
        if self.has_rule(ALL_WILDCARD) {
            return;
        }

        // add dummy rule that allows any production
        self.rules
            .insert(ALL_WILDCARD, Rule::new(ALL_WILDCARD, &[]));
    }
}

impl Debug for RuleSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (name, rule) in &self.rules {
            let mut dst = String::new();
            for (pos, d) in rule.rhs().iter().enumerate() {
                if pos > 0 {
                    dst.push_str(" | ");
                }
                dst.push_str(*d);
            }
            if dst.is_empty() {
                continue;
            }
            write!(f, "{name} -> {dst}\n")?;
        }

        Ok(())
    }
}
