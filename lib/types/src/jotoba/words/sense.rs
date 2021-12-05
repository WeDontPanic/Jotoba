use crate::jotoba::languages::Language;

use super::{
    dialect::Dialect,
    field::Field,
    foreign_language::ForeignLanguage,
    gtype::GType,
    misc::Misc,
    part_of_speech::{PartOfSpeech, PosSimple},
};
use serde::{Deserialize, Serialize};

#[cfg(feature = "jotoba_intern")]
use localization::{language::Language as LocLanguage, traits::Translatable, TranslationDict};

/// A single sense for a word. Represents one language,
/// one misc item and 1..n glosses
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize, Hash)]
pub struct Sense {
    pub id: u8,
    pub misc: Option<Misc>,
    pub field: Option<Field>,
    pub dialect: Option<Dialect>,
    pub glosses: Vec<Gloss>,
    pub xref: Option<String>,
    pub antonym: Option<String>,
    pub information: Option<String>,
    pub part_of_speech: Vec<PartOfSpeech>,
    pub language: Language,
    pub example_sentence: Option<u32>,
    pub gairaigo: Option<Gairaigo>,
}

#[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize, Hash)]
pub struct Gairaigo {
    pub language: ForeignLanguage,
    pub fully_derived: bool,
    pub original: String,
}

impl Eq for Sense {}

/// A gloss value represents one word in the
/// translated language.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize, Hash)]
pub struct Gloss {
    pub gloss: String,
    pub g_type: Option<GType>,
    pub occurrence: u32,
}

impl Sense {
    /// Get all pos_simple of a sense
    pub fn get_pos_simple(&self) -> Vec<PosSimple> {
        let mut pos_simple = self
            .part_of_speech
            .iter()
            .map(|i| i.to_pos_simple())
            .flatten()
            .collect::<Vec<_>>();

        pos_simple.sort_unstable();
        pos_simple.dedup();
        pos_simple
    }
}

// Jotoba intern only features
#[cfg(feature = "jotoba_intern")]
impl Sense {
    /// Get a senses tags prettified
    #[inline]
    pub fn get_glosses(&self) -> String {
        use itertools::Itertools;
        self.glosses.iter().map(|i| i.gloss.clone()).join("; ")
    }

    /// Returns an `xref` of the sense if available
    #[inline]
    pub fn get_xref(&self) -> Option<&str> {
        self.xref.as_ref().and_then(|xref| xref.split('・').next())
    }

    /// Returns an `antonym` of the sense if available
    #[inline]
    pub fn get_antonym(&self) -> Option<&str> {
        self.antonym
            .as_ref()
            .and_then(|antonym| antonym.split('・').next())
    }

    // Get a senses tags prettified
    pub fn get_parts_of_speech(&self, dict: &TranslationDict, language: LocLanguage) -> String {
        use itertools::Itertools;
        self.part_of_speech
            .iter()
            .map(|i| i.gettext_custom(dict, Some(language)))
            .join(", ")
    }

    pub fn get_infos(
        &self,
        dict: &TranslationDict,
        language: LocLanguage,
    ) -> Option<(
        Option<String>,
        Option<&str>,
        Option<&str>,
        Option<Dialect>,
        Option<String>,
    )> {
        let info_str = self.get_information_string(dict, language);
        let xref = self.get_xref();
        let antonym = self.get_antonym();
        let dialect = self.dialect;

        if xref.is_none() && info_str.is_none() && antonym.is_none() && self.gairaigo.is_none() {
            None
        } else {
            let gairaigo_txt = self.get_gairaigo(dict, language);
            Some((info_str, xref, antonym, dialect, gairaigo_txt))
        }
    }

    fn get_gairaigo(&self, dict: &TranslationDict, language: LocLanguage) -> Option<String> {
        self.gairaigo.as_ref().map(|gairaigo| {
            let lang = gairaigo
                .language
                .pgettext(dict, "foreign_lang", Some(language));
            dict.gettext_fmt("From {}: {}", &[lang, &gairaigo.original], Some(language))
        })
    }

    /// Return human readable information about a gloss
    pub fn get_information_string(
        &self,
        dict: &TranslationDict,
        language: LocLanguage,
    ) -> Option<String> {
        use itertools::Itertools;
        let arr: [Option<String>; 3] = [
            self.misc
                .map(|i| i.gettext(dict, Some(language)).to_owned()),
            self.field.map(|i| i.gettext_custom(dict, Some(language))),
            self.information.clone(),
        ];

        let res = arr
            .iter()
            .filter_map(|i| i.is_some().then(|| i.as_ref().unwrap()))
            .collect::<Vec<_>>();

        if res.is_empty() {
            return None;
        }

        if self.xref.is_some() || self.antonym.is_some() {
            Some(format!("{}.", res.iter().join(", ")))
        } else {
            Some(res.iter().join(", "))
        }
    }
}
