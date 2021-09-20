include!(concat!(env!("OUT_DIR"), "/templates.rs"));

#[macro_use]
mod actix_ructe;

pub mod about;
pub mod index;
mod pagination;
pub mod search_ep;
mod session;
mod url_query;
pub mod user_settings;
pub mod web_error;

use std::fmt::Display;

use localization::{
    language::Language,
    traits::{Translatable, TranslatablePlural},
    TranslationDict,
};
use pagination::Pagination;
use resources::models::names::Name;
use search::query::Query;

use search::{
    kanji::result::Item as KanjiItem, query::UserSettings, query_parser::QueryType,
    sentence::result::Item as SentenceItem, word::result::WordResult,
};

/// Data for the base template
pub struct BaseData<'a> {
    pub site: Site<'a>,
    pub dict: &'a TranslationDict,
    pub user_settings: UserSettings,
    pub pagination: Option<Pagination>,
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

/// The particular search result items
#[derive(Clone)]
pub enum ResultData {
    Word(WordResult),
    KanjiInfo(Vec<KanjiItem>),
    Name(Vec<Name>),
    Sentence(Vec<SentenceItem>),
}

impl<'a> BaseData<'a> {
    #[inline]
    pub fn new(dict: &'a TranslationDict, user_settings: UserSettings) -> Self {
        Self {
            site: Site::Index,
            dict,
            user_settings,
            pagination: None,
        }
    }

    #[inline]
    pub fn with_site(mut self, site: Site<'a>) -> Self {
        self.site = site;
        self
    }

    #[inline]
    pub fn with_pages(&mut self, items: u32, curr_page: u32) {
        let mut pagination = Pagination {
            items,
            curr_page,
            items_per_page: self.user_settings.items_per_page,
        };

        // Don't show paginator if there is only one page
        if pagination.get_last() == 1 {
            return;
        }

        if curr_page > pagination.get_last() {
            pagination.curr_page = pagination.get_last();
        }

        self.pagination = Some(pagination);
    }

    #[inline]
    pub fn get_search_site_id(&self) -> u8 {
        if let Site::SearchResult(ref res) = self.site {
            return match res.result {
                ResultData::Word(_) => 0,
                ResultData::KanjiInfo(_) => 1,
                ResultData::Sentence(_) => 2,
                ResultData::Name(_) => 3,
            };
        }

        0
    }

    #[inline]
    pub fn get_search_site_name(&self) -> &str {
        if let Site::SearchResult(ref res) = self.site {
            return match res.result {
                ResultData::Word(_) => self.gettext("Words"),
                ResultData::KanjiInfo(_) => self.gettext("Kanji"),
                ResultData::Sentence(_) => self.gettext("Sentences"),
                ResultData::Name(_) => self.gettext("Names"),
            };
        }

        self.gettext("Words")
    }

    #[inline]
    pub fn with_search_result(self, query: &'a Query, result: ResultData) -> Self {
        let search_result = SearchResult { query, result };
        self.with_site(Site::SearchResult(search_result))
    }

    /// Gets an owned String of the query
    pub fn get_query_str(&self) -> String {
        let query = match &self.site {
            Site::SearchResult(search_result) => {
                Some(search_result.query.without_search_type_tags())
            }
            _ => None,
        }
        .unwrap_or_default();
        println!("query_str: {}", query);
        query
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
}

/// Translation helper
impl<'a> BaseData<'a> {
    #[inline]
    pub fn get_lang(&self) -> Language {
        self.user_settings.page_lang
    }

    #[inline]
    pub fn gettext<T: Translatable>(&self, t: T) -> &'a str {
        t.gettext(&self.dict, Some(self.get_lang()))
    }

    #[inline]
    pub fn gettext_custom<T: Translatable>(&self, t: T) -> String {
        t.gettext_custom(&self.dict, Some(self.get_lang()))
    }

    #[inline]
    pub fn pgettext<T: Translatable>(&self, t: T, context: &'a str) -> &'a str {
        t.pgettext(&self.dict, context, Some(self.get_lang()))
    }

    #[inline]
    pub fn ngettext<T: TranslatablePlural>(&self, t: T, n: u64) -> &'a str {
        t.ngettext(&self.dict, n, Some(self.get_lang()))
    }

    #[inline]
    pub fn pngettext<T: TranslatablePlural>(&self, t: T, context: &'a str, n: u64) -> &'a str {
        t.npgettext(&self.dict, context, n, Some(self.get_lang()))
    }

    // Format functions

    #[inline]
    pub fn gettext_fmt<T: Translatable, V: Display + Sized>(&self, t: T, values: &[V]) -> String {
        t.gettext_fmt(&self.dict, values, Some(self.get_lang()))
    }

    #[inline]
    pub fn pgettext_fmt<T: Translatable, V: Display + Sized>(
        &self,
        t: T,
        context: &'a str,
        values: &[V],
    ) -> String {
        t.pgettext_fmt(&self.dict, context, values, Some(self.get_lang()))
    }

    #[inline]
    pub fn ngettext_fmt<T: TranslatablePlural, V: Display + Sized>(
        &self,
        t: T,
        n: u64,
        values: &[V],
    ) -> String {
        t.ngettext_fmt(&self.dict, n, values, Some(self.get_lang()))
    }

    #[inline]
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
