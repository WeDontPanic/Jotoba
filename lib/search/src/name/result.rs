use itertools::Itertools;
use localization::{language::Language, traits::Translatable, TranslationDict};
use types::jotoba::names::Name;

use crate::engine::result::SearchResult;

pub struct NameResult {
    pub items: Vec<&'static Name>,
    pub total_count: u32,
}

/// Returns the Name's types in an human readable way
pub fn get_types_humanized(name: &Name, dict: &TranslationDict, lang: Language) -> String {
    if let Some(ref n_types) = name.name_type {
        n_types
            .iter()
            .filter_map(|i| (!i.is_gender()).then(|| i.pgettext(dict, "name_type", Some(lang))))
            .join(", ")
    } else {
        String::from("")
    }
}

impl From<SearchResult<&'static Name>> for NameResult {
    #[inline]
    fn from(res: SearchResult<&'static Name>) -> Self {
        let items: Vec<_> = res.items.into_iter().map(|i| i.item).collect();
        NameResult {
            total_count: res.total_items as u32,
            items,
        }
    }
}
