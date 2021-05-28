/// How db entries should be matched with
/// the query in order to be valid as result
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum SearchMode {
    Exact,
    Variable,
    LeftVariable,
    RightVariable,
}

impl SearchMode {
    /// Compares a string based on the mode and case
    pub fn str_eq<S: AsRef<str>>(&self, a: S, b: S, ign_case: bool) -> bool {
        let (a, b) = if ign_case {
            (a.as_ref().to_lowercase(), b.as_ref().to_lowercase())
        } else {
            (a.as_ref().to_owned(), b.as_ref().to_owned())
        };

        match *self {
            SearchMode::Exact => a == b,
            SearchMode::Variable => a.contains(&b),
            SearchMode::LeftVariable => a.starts_with(&b),
            SearchMode::RightVariable => a.ends_with(&b),
        }
    }

    /// Returns a string which can be placed inside a like query
    pub fn to_like<S: AsRef<str>>(&self, a: S) -> String {
        match self {
            SearchMode::Exact => a.as_ref().to_owned(),
            SearchMode::Variable => format!("%{}%", a.as_ref()),
            SearchMode::LeftVariable => format!("%{}", a.as_ref()),
            SearchMode::RightVariable => format!("{}%", a.as_ref()),
        }
    }
}
