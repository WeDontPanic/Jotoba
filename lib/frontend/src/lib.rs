include!(concat!(env!("OUT_DIR"), "/templates.rs"));

use std::fmt::Display;

use localization::language::Language;
use localization::traits::Translatable;
use localization::traits::TranslatablePlural;
use localization::TranslationDict;
use models::name::Name;
use parse::jmdict;
use search::query::Query;

#[macro_use]
mod actix_ructe;

pub mod about;
pub mod index;
pub mod search_ep;
pub mod web_error;

use search::query::UserSettings;
use search::{kanji::result::Item as KanjiItem, sentence::result::Item as SentenceItem};
use search::{query_parser::QueryType, word::result::WordResult};

/// Data for the base template
pub struct BaseData<'a> {
    pub site: Site<'a>,
    pub dict: &'a TranslationDict,
    pub user_settings: UserSettings,
}

/// The site to display
#[derive(Clone)]
pub enum Site<'a> {
    SearchResult(SearchResult<'a>),
    Index,
    About,
}

/// Search result data. Required by individual templates to render the result items
#[derive(Clone)]
pub struct SearchResult<'a> {
    pub query: &'a Query,
    pub result: ResultData,
}

#[derive(Clone)]
/// The particular search result items
pub enum ResultData {
    Word(WordResult),
    KanjiInfo(Vec<KanjiItem>),
    Name(Vec<Name>),
    Sentence(Vec<SentenceItem>),
}

impl<'a> BaseData<'a> {
    pub fn new(site: Site<'a>, dict: &'a TranslationDict, user_settings: UserSettings) -> Self {
        Self {
            site,
            dict,
            user_settings,
        }
    }

    pub fn new_word_search(
        query: &'a Query,
        result: WordResult,
        locale_dict: &'a TranslationDict,
        user_settings: UserSettings,
    ) -> Self {
        Self::new_search_result(query, ResultData::Word(result), locale_dict, user_settings)
    }

    pub fn new_kanji_search(
        query: &'a Query,
        result: Vec<KanjiItem>,
        locale_dict: &'a TranslationDict,
        user_settings: UserSettings,
    ) -> Self {
        Self::new_search_result(
            query,
            ResultData::KanjiInfo(result),
            locale_dict,
            user_settings,
        )
    }

    pub fn new_name_search(
        query: &'a Query,
        result: Vec<Name>,
        locale_dict: &'a TranslationDict,
        user_settings: UserSettings,
    ) -> Self {
        Self::new_search_result(query, ResultData::Name(result), locale_dict, user_settings)
    }

    pub fn new_sentence_search(
        query: &'a Query,
        result: Vec<SentenceItem>,
        locale_dict: &'a TranslationDict,
        user_settings: UserSettings,
    ) -> Self {
        Self::new_search_result(
            query,
            ResultData::Sentence(result),
            locale_dict,
            user_settings,
        )
    }

    /// Gets an owned String of the query
    pub fn get_query_str(&self) -> String {
        match &self.site {
            Site::SearchResult(search_result) => {
                Some(search_result.query.without_search_type_tags())
            }
            _ => None,
        }
        .unwrap_or_default()
    }

    /// Return a string 'selected' if the query_type in qs is equal to i
    pub fn sel_str(&self, i: QueryType) -> &'static str {
        let is_selected = match &self.site {
            Site::SearchResult(search_result) => search_result.query.type_ == i,
            _ => false,
        };

        if is_selected {
            "selected"
        } else {
            ""
        }
    }

    fn new_search_result(
        query: &'a Query,
        result: ResultData,
        locale_dict: &'a TranslationDict,
        user_settings: UserSettings,
    ) -> Self {
        let search_result = SearchResult { query, result };
        Self::new(
            Site::SearchResult(search_result),
            locale_dict,
            user_settings,
        )
    }
}

/// Translation helper
impl<'a> BaseData<'a> {
    pub fn get_lang(&self) -> Language {
        match self.user_settings.user_lang {
            jmdict::languages::Language::German => Language::German,
            jmdict::languages::Language::English => Language::English,
            jmdict::languages::Language::Russain => Language::Russain,
            jmdict::languages::Language::Spanish => Language::Spanish,
            jmdict::languages::Language::Swedish => Language::Swedish,
            jmdict::languages::Language::French => Language::French,
            jmdict::languages::Language::Dutch => Language::Dutch,
            jmdict::languages::Language::Hungarian => Language::Hungarian,
            jmdict::languages::Language::Slovenian => Language::Slovenian,
        }
    }

    pub fn gettext<T: Translatable>(&self, t: T) -> &'a str {
        t.gettext(&self.dict, Some(self.get_lang()))
    }

    pub fn gettext_custom<T: Translatable>(&self, t: T) -> String {
        t.gettext_custom(&self.dict, Some(self.get_lang()))
    }

    pub fn pgettext<T: Translatable>(&self, t: T, context: &'a str) -> &'a str {
        t.pgettext(&self.dict, context, Some(self.get_lang()))
    }

    pub fn ngettext<T: TranslatablePlural>(&self, t: T, n: u64) -> &'a str {
        t.ngettext(&self.dict, n, Some(self.get_lang()))
    }

    pub fn pngettext<T: TranslatablePlural>(&self, t: T, context: &'a str, n: u64) -> &'a str {
        t.npgettext(&self.dict, context, n, Some(self.get_lang()))
    }

    // Format functions

    pub fn gettext_fmt<T: Translatable, V: Display + Sized>(&self, t: T, values: &[V]) -> String {
        t.gettext_fmt(&self.dict, values, Some(self.get_lang()))
    }

    pub fn pgettext_fmt<T: Translatable, V: Display + Sized>(
        &self,
        t: T,
        context: &'a str,
        values: &[V],
    ) -> String {
        t.pgettext_fmt(&self.dict, context, values, Some(self.get_lang()))
    }

    pub fn ngettext_fmt<T: TranslatablePlural, V: Display + Sized>(
        &self,
        t: T,
        n: u64,
        values: &[V],
    ) -> String {
        t.ngettext_fmt(&self.dict, n, values, Some(self.get_lang()))
    }

    pub fn pngettext_fmt<T: TranslatablePlural, V: Display + Sized>(
        &self,
        t: T,
        context: &'a str,
        n: u64,
        values: &[V],
    ) -> String {
        t.npgettext_fmt(&self.dict, context, n, values, Some(self.get_lang()))
    }
}
