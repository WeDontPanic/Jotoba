use crate::parse::jmdict::{
    dialect::Dialect,
    field::Field,
    gtype::GType,
    languages::Language,
    misc::Misc,
    part_of_speech::{PartOfSpeech, PosSimple},
};
use itertools::Itertools;
use localization::{language::Language as LocLanguage, traits::Translatable, TranslationDict};
use serde::{Deserialize, Serialize};

/// A single sense for a word. Represents one language,
/// one misc item and 1..n glosses
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct Sense {
    pub misc: Option<Misc>,
    pub field: Option<Field>,
    pub dialect: Option<Dialect>,
    pub glosses: Vec<Gloss>,
    pub xref: Option<String>,
    pub antonym: Option<String>,
    pub information: Option<String>,
    pub part_of_speech: Vec<PartOfSpeech>,
    pub language: Language,
}

/// A gloss value represents one word in the
/// translated language.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct Gloss {
    pub gloss: String,
    pub g_type: Option<GType>,
}

impl Sense {
    /// Get a senses tags prettified
    #[inline]
    pub fn get_glosses(&self) -> String {
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

    // Get a senses tags prettified
    pub fn get_parts_of_speech(&self, dict: &TranslationDict, language: LocLanguage) -> String {
        self.part_of_speech
            .iter()
            .map(|i| i.gettext_custom(dict, Some(language)))
            .join(", ")
    }

    pub fn get_infos(
        &self,
        dict: &TranslationDict,
        language: LocLanguage,
    ) -> Option<(Option<String>, Option<&str>, Option<&str>, Option<Dialect>)> {
        let info_str = self.get_information_string(dict, language);
        let xref = self.get_xref();
        let antonym = self.get_antonym();
        let dialect = self.dialect;

        if xref.is_none() && info_str.is_none() && antonym.is_none() {
            None
        } else {
            Some((info_str, xref, antonym, dialect))
        }
    }

    /// Return human readable information about a gloss
    pub fn get_information_string(
        &self,
        dict: &TranslationDict,
        language: LocLanguage,
    ) -> Option<String> {
        let arr: [Option<String>; 3] = [
            self.misc
                .map(|i| i.gettext(dict, Some(language)).to_owned()),
            self.field.map(|i| i.gettext_custom(dict, Some(language))),
            self.information.clone(),
        ];

        let res = arr
            .iter()
            .filter_map(|i| i.is_some().then(|| i.as_ref().unwrap()))
            .collect_vec();

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
