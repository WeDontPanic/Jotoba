use models::name::Name;
use search::query::Query;

#[macro_use]

mod actix_ructe;

pub mod about;
pub mod api;
pub mod index;
pub mod search_ep;
pub mod web_error;

use search::{kanji::result::Item as KanjiItem, sentence::result::Item as SentenceItem};
use search::{query_parser::QueryType, word::result::WordResult};

/// Data for the base template
pub struct BaseData<'a> {
    pub site: Site<'a>,
}

#[derive(Clone)]
/// The site to display
pub enum Site<'a> {
    SearchResult(SearchResult<'a>),
    Index,
    About,
}

/// Result data of a search
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
    pub fn new(site: Site<'a>) -> Self {
        Self { site }
    }

    pub fn new_word_search(query: &'a Query, result: WordResult) -> Self {
        Self::new_search_result(query, ResultData::Word(result))
    }

    pub fn new_kanji_search(query: &'a Query, result: Vec<KanjiItem>) -> Self {
        Self::new_search_result(query, ResultData::KanjiInfo(result))
    }

    pub fn new_name_search(query: &'a Query, result: Vec<Name>) -> Self {
        Self::new_search_result(query, ResultData::Name(result))
    }

    pub fn new_sentence_search(query: &'a Query, result: Vec<SentenceItem>) -> Self {
        Self::new_search_result(query, ResultData::Sentence(result))
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

    fn new_search_result(query: &'a Query, result: ResultData) -> Self {
        let search_result = SearchResult { result, query };
        Self::new(Site::SearchResult(search_result))
    }
}
