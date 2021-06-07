use strum_macros::AsRefStr;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, AsRefStr)]
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
    Imperative,
    Tai,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SentencePart {
    pub text: String,
    pub info: Option<String>,
    pub furigana: Option<String>,
    pub pos: i32,
}
