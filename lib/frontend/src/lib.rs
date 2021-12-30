include!(concat!(env!("OUT_DIR"), "/templates.rs"));

#[macro_use]
mod actix_ructe;

pub mod about;
pub mod example_sentence;
pub mod help_page;
pub mod index;
pub mod news;
mod pagination;
pub mod search_ep;
pub mod search_help;
mod session;
pub mod templ_utils;
pub mod unescaped;
mod url_query;
pub mod user_settings;
pub mod web_error;

use std::fmt::Display;

use config::Config;
use localization::{
    language::Language,
    traits::{Translatable, TranslatablePlural},
    TranslationDict,
};
use pagination::Pagination;
use resources::news::NewsEntry;
use search::{query::Query, sentence::result::SentenceResult};

use search::{kanji::result::Item as KanjiItem, query::UserSettings, word::result::WordResult};
use search_help::SearchHelp;
use types::jotoba::{names::Name, search::QueryType};
use unescaped::{UnescapedStr, UnescapedString};

/// Data for the base template
pub struct BaseData<'a> {
    pub site: Site<'a>,
    pub dict: &'a TranslationDict,
    pub user_settings: UserSettings,
    pub pagination: Option<Pagination>,
    pub asset_hash: &'a str,
    pub config: &'a Config,
}

/// The site to display
#[derive(Clone)]
pub enum Site<'a> {
    SearchResult(SearchResult<'a>),
    Index,
    About,
    InfoPage,
    News(Vec<&'static NewsEntry>),
}

/// Search result data. Required by individual templates to render the result items
#[derive(Clone)]
pub struct SearchResult<'a> {
    pub query: &'a Query,
    pub result: ResultData,
    pub search_help: Option<SearchHelp>,
}

/// The particular search result items
#[derive(Clone)]
pub enum ResultData {
    Word(WordResult),
    KanjiInfo(Vec<KanjiItem>),
    Name(Vec<&'static Name>),
    Sentence(SentenceResult),
}

impl<'a> BaseData<'a> {
    #[inline]
    pub fn new(
        dict: &'a TranslationDict,
        user_settings: UserSettings,
        asset_hash: &'a str,
        config: &'a Config,
    ) -> Self {
        Self {
            site: Site::Index,
            dict,
            user_settings,
            pagination: None,
            asset_hash,
            config,
        }
    }

    #[inline]
    pub fn with_site(mut self, site: Site<'a>) -> Self {
        self.site = site;
        self
    }

    #[inline]
    pub fn with_cust_pages(
        &mut self,
        items: u32,
        curr_page: u32,
        items_per_page: u32,
        max_pages: u32,
    ) {
        let mut pagination = Pagination {
            items,
            curr_page,
            items_per_page,
            max_pages,
        };

        // Don't show paginator if there is only one or no page
        if pagination.get_last() <= 1 {
            return;
        }

        if curr_page > pagination.get_last() {
            pagination.curr_page = pagination.get_last();
        }

        self.pagination = Some(pagination);
    }

    #[inline]
    pub fn with_pages(&mut self, items: u32, curr_page: u32) {
        self.with_cust_pages(items, curr_page, self.user_settings.page_size, 100);
    }

    #[inline]
    pub fn get_search_help(&self) -> Option<&SearchHelp> {
        let help = self.site.as_search_result()?.search_help.as_ref()?;
        (!help.is_empty()).then(|| help)
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
                ResultData::Word(_) => self.gettext("Words").as_str(),
                ResultData::KanjiInfo(_) => self.gettext("Kanji").as_str(),
                ResultData::Sentence(_) => self.gettext("Sentences").as_str(),
                ResultData::Name(_) => self.gettext("Names").as_str(),
            };
        }

        self.gettext("Words").as_str()
    }

    #[inline]
    pub fn with_search_result(
        self,
        query: &'a Query,
        result: ResultData,
        search_help: Option<SearchHelp>,
    ) -> Self {
        let search_result = SearchResult {
            query,
            result,
            search_help,
        };
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

    /// Returns true if the kanji compounds should be collapsed by default
    pub fn kanji_copounds_collapsed(&self) -> bool {
        self.pagination.as_ref().map(|i| i.get_last()).unwrap_or(0) > 1
    }
}

impl<'a> Site<'a> {
    #[inline]
    pub fn as_search_result(&self) -> Option<&SearchResult<'a>> {
        if let Self::SearchResult(v) = self {
            Some(v)
        } else {
            None
        }
    }
}

impl ResultData {
    /// Returns `true` if the ResultData does not contain any items
    #[inline]
    pub fn is_empty(&self) -> bool {
        match self {
            ResultData::Word(w) => w.items.is_empty(),
            ResultData::KanjiInfo(k) => k.is_empty(),
            ResultData::Name(n) => n.is_empty(),
            ResultData::Sentence(s) => s.items.is_empty(),
        }
    }
}

impl<'a> SearchResult<'a> {
    pub(crate) fn og_tag_info(&self) -> String {
        format!("{} results. See more...", self.result_count())
    }

    pub(crate) fn search_type_ogg(&self) -> &'static str {
        match self.result {
            ResultData::Word(_) => "words",
            ResultData::KanjiInfo(_) => "kanji",
            ResultData::Sentence(_) => "sentences",
            ResultData::Name(_) => "names",
        }
    }

    fn result_count(&self) -> usize {
        match &self.result {
            ResultData::Word(w) => w.count,
            ResultData::KanjiInfo(k) => k.len(),
            ResultData::Name(n) => n.len(),
            ResultData::Sentence(s) => s.items.len(),
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
    pub fn gettext<T: Translatable>(&self, t: T) -> UnescapedStr<'a> {
        t.gettext(&self.dict, Some(self.get_lang())).into()
    }

    #[inline]
    pub fn gettext_custom<T: Translatable>(&self, t: T) -> UnescapedString {
        t.gettext_custom(&self.dict, Some(self.get_lang())).into()
    }

    #[inline]
    pub fn pgettext<T: Translatable>(&self, t: T, context: &'a str) -> UnescapedStr<'a> {
        t.pgettext(&self.dict, context, Some(self.get_lang()))
            .into()
    }

    #[inline]
    pub fn ngettext<T: TranslatablePlural>(&self, t: T, n: u64) -> UnescapedStr<'a> {
        t.ngettext(&self.dict, n, Some(self.get_lang())).into()
    }

    #[inline]
    pub fn pngettext<T: TranslatablePlural>(
        &self,
        t: T,
        context: &'a str,
        n: u64,
    ) -> UnescapedStr<'a> {
        t.npgettext(&self.dict, context, n, Some(self.get_lang()))
            .into()
    }

    // Format functions

    #[inline]
    pub fn gettext_fmt<T: Translatable, V: Display + Sized + Clone>(
        &self,
        t: T,
        values: &[V],
    ) -> UnescapedString {
        t.gettext_fmt(&self.dict, values, Some(self.get_lang()))
            .into()
    }

    #[inline]
    pub fn pgettext_fmt<T: Translatable, V: Display + Sized + Clone>(
        &self,
        t: T,
        context: &'a str,
        values: &[V],
    ) -> UnescapedString {
        t.pgettext_fmt(&self.dict, context, values, Some(self.get_lang()))
            .into()
    }

    #[inline]
    pub fn ngettext_fmt<T: TranslatablePlural, V: Display + Sized + Clone>(
        &self,
        t: T,
        n: u64,
        values: &[V],
    ) -> UnescapedString {
        t.ngettext_fmt(&self.dict, n, values, Some(self.get_lang()))
            .into()
    }

    #[inline]
    pub fn pngettext_fmt<T: TranslatablePlural, V: Display + Sized + Clone>(
        &self,
        t: T,
        context: &'a str,
        n: u64,
        values: &[V],
    ) -> UnescapedString {
        t.npgettext_fmt(&self.dict, context, n, values, Some(self.get_lang()))
            .into()
    }

    #[inline]
    pub fn gt_search_link<T: Translatable, V: Display + Sized + Clone>(
        &self,
        t: T,
        value: V,
    ) -> UnescapedString {
        let link = format_search_link(value);
        t.gettext_fmt(&self.dict, &[link], Some(self.get_lang()))
            .into()
    }

    #[inline]
    pub fn gt_search_links<T: Translatable, V: Display + Sized + Clone>(
        &self,
        t: T,
        link: usize,
        values: &[V],
    ) -> UnescapedString {
        let mut values = values.iter().map(|i| i.to_string()).collect::<Vec<_>>();
        values[link] = format_search_link(&values[link]);
        t.gettext_fmt(&self.dict, &values, Some(self.get_lang()))
            .into()
    }

    #[inline]
    pub fn ngt_search_links<T: TranslatablePlural, V: Display + Sized + Clone>(
        &self,
        t: T,
        link: usize,
        values: &[V],
        n: u64,
    ) -> UnescapedString {
        let mut values = values.iter().map(|i| i.to_string()).collect::<Vec<_>>();
        values[link] = format_search_link(&values[link]);
        t.ngettext_fmt(&self.dict, n, &values, Some(self.get_lang()))
            .into()
    }
}

fn format_search_link<V: Display + Sized + Clone>(input: V) -> String {
    format!(
        "<a class='clickable no-align green' href='/search/{}'>{}</a>",
        input, input
    )
}
