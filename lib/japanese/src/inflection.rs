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
    pub info: Option<&'static str>,
    pub furigana: Option<String>,
    pub pos: i32,
    pub add_class: Option<String>,
}

impl SentencePart {
    pub fn get_add_class(&self) -> String {
        self.add_class
            .as_ref()
            .map(|i| i.clone())
            .unwrap_or_default()
    }
}
