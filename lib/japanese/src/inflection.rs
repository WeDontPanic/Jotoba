use localization::traits::Translatable;
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
    pub furi_guessed: bool,
    pub pos: i32,
    pub add_class: Option<String>,
    pub lexeme: String,
}

impl SentencePart {
    pub fn get_add_class(&self) -> String {
        self.add_class.as_ref().cloned().unwrap_or_default()
    }
}

impl Translatable for Inflection {
    fn get_id(&self) -> &'static str {
        match self {
            Inflection::Negative => "Negative",
            Inflection::Polite => "Polite",
            Inflection::Present => "Present",
            Inflection::Past => "Past",
            Inflection::TeForm => "TeForm",
            Inflection::Potential => "Potential",
            Inflection::Passive => "Passive",
            Inflection::Causative => "Causative",
            Inflection::CausativePassive => "CausativePassive",
            Inflection::Imperative => "Imperative",
            Inflection::Tai => "Tai",
        }
    }

    fn gettext<'a>(
        &self,
        dict: &'a localization::TranslationDict,
        language: Option<localization::language::Language>,
    ) -> &'a str {
        self.pgettext(dict, "inflection", language)
    }
}
