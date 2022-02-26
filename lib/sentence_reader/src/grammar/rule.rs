use super::rule_set::ALL_WILDCARD;

/// Represents a single rule describing a possible production
/// of a grammar
#[derive(Clone, Copy)]
pub struct Rule {
    name: &'static str,
    rhs: &'static [&'static str],
}

impl Rule {
    /// Creates a new rule
    pub fn new(name: &'static str, rhs: &'static [&'static str]) -> Self {
        Self { name, rhs }
    }

    /// Get the rule's name.
    #[inline]
    pub fn name(&self) -> &'static str {
        self.name
    }

    /// Get the rule's destination rules
    #[inline]
    pub fn rhs(&self) -> &'static [&'static str] {
        self.rhs
    }

    /// Returns `true` if the rule has a dst rule with `name`
    #[inline]
    pub fn has_dst(&self, name: &str) -> bool {
        self.rhs.iter().any(|i| *i == name || *i == ALL_WILDCARD)
    }
}

pub trait ToRule {
    fn to_rule(&self) -> Option<&str>;
}

impl ToRule for &'static str {
    #[inline]
    fn to_rule(&self) -> Option<&str> {
        Some(self)
    }
}

impl<T: ToRule> ToRule for &T {
    #[inline]
    fn to_rule(&self) -> Option<&str> {
        (*self).to_rule()
    }
}
