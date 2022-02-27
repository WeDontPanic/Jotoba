#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Inflection {
    Negative,
    Polite,
    Present,
    Past,
    TeForm,
    Potential,
    Passive,
    Causative,
    PotentialOrPassive,
    Imperative,
    Tai,
    TeIru,
}

#[cfg(feature = "jotoba_intern")]
impl localization::traits::Translatable for Inflection {
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
            // TODO: rename in translations
            Inflection::PotentialOrPassive => "CausativePassive",
            Inflection::Imperative => "Imperative",
            Inflection::Tai => "Tai",
            // new:
            Inflection::TeIru => "TeIru",
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
