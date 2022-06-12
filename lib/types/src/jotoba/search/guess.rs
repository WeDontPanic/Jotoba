use serde::{Deserialize, Serialize};

/// A guess representing structure. Gives some vague information about the relation to the
/// actual value i.e if its likely to be exact, less, etc..
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Guess {
    pub value: u32,
    pub guess_type: GuessType,
}

/// Vague guess relation to a guesses actual value
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum GuessType {
    Accurate,
    MoreThan,
    LessThan,
    Undefined,
}

impl Guess {
    /// Creates a new `Guess`
    #[inline]
    pub fn new(value: u32, guess_type: GuessType) -> Self {
        Self { value, guess_type }
    }

    /// Formats the guess to a human readable string
    pub fn format(&self) -> String {
        let prefix = self.guess_type.get_prefix();
        format!("{}{}", prefix, self.value)
    }
}

impl GuessType {
    #[inline]
    pub fn get_prefix(&self) -> &'static str {
        match self {
            GuessType::Accurate => "",
            GuessType::Undefined => "",
            GuessType::MoreThan => ">",
            GuessType::LessThan => "<",
        }
    }
}
