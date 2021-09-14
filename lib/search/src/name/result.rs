use itertools::Itertools;
use localization::{language::Language, traits::Translatable, TranslationDict};
use resources::models::names::Name;

pub struct NameResult {
    pub items: Vec<Name>,
    pub total_count: u32,
}

/// Returns the Name's types in an human readable way
pub fn get_types_humanized(name: &Name, dict: &TranslationDict, lang: Language) -> String {
    if let Some(ref n_types) = name.name_type {
        n_types
            .iter()
            .filter_map(|i| (!i.is_gender()).then(|| i.gettext(dict, Some(lang))))
            .join(", ")
    } else {
        String::from("")
    }
}
