mod binary_search;
mod jaro_search;
pub mod store_item;
pub mod text_store;

use std::{cmp::Ordering, collections::HashMap};

use binary_search::Search as BinarySearch;
use jaro_search::Search as JaroSearch;
use parse::jmdict::languages::Language;
use strsim::jaro_winkler;
use text_store::TextStore;
use utils::diff;

use self::{jaro_search::AsyncSearch, store_item::Item};

/// Searches for kanji suggestions by their meanings
pub async fn kanji_meaning<'a, T: TextStore>(
    dict: &'a TextSearch<T>,
    query: &'a str,
) -> Option<Vec<&'a T::Item>> {
    let mut items: Vec<_> = dict.find_binary(query.to_owned()).take(100).collect();

    if items.len() < 5 {
        let jaro_res = dict.find_jaro_async(query, 5).await;
        items.extend(jaro_res);
    }

    items.sort_by(|a, b| result_order::<T>(a, b, query));

    Some(items)
}

#[derive(Clone)]
pub struct SuggestionSearch<T: TextStore> {
    dicts: HashMap<Language, TextSearch<T>>,
}

impl<T: TextStore> SuggestionSearch<T> {
    pub fn new(text_store: HashMap<Language, TextSearch<T>>) -> Self {
        Self { dicts: text_store }
    }

    /// Searches for suggestions in the provided language and uses english as fallback
    pub async fn search<'a>(&'a self, query: &'a str, lang: Language) -> Option<Vec<&'a T::Item>> {
        if query.is_empty() {
            return None;
        }

        let mut res: Vec<&T::Item> = self.do_search(query, lang).unwrap_or_default();

        if res.len() < 5 {
            // Search for english
            res.extend(self.do_search(query, Language::English).unwrap_or_default());

            // Do jaro search
            if query.len() > 3 {
                let dict = self.dicts.get(&lang)?;
                res.extend(dict.find_jaro_async(query, 3).await);
            }
        }

        res.sort_by(|a, b| result_order::<T>(a, b, query));

        // Try to remove completely trash results
        let mut res: Vec<_> = res
            .into_iter()
            .filter(|i| result_order_value(query, i.get_text()) >= 55)
            .collect();

        // Remove duplicates
        res.dedup_by(|a, b| a.get_hash() == b.get_hash() || a.get_text() == b.get_text());

        Some(res)
    }

    /// Searches for one language
    fn do_search<'a>(&'a self, query: &'a str, lang: Language) -> Option<Vec<&'a T::Item>> {
        let dict = self.dicts.get(&lang)?;

        let mut res: Vec<_> = dict.find_binary(query.to_owned()).take(100).collect();

        // Also search for 1st one with uppercase
        if query.chars().next().unwrap().is_lowercase() {
            res.extend(dict.find_binary(utils::first_letter_upper(query)).take(100));
        }

        Some(res)
    }
}

// Order by best match against `query`
// TODO don't use jaro_winkler algorithm within order function since its way to heavy
// Idea: calculate jaro_winkler for each entry once and then use this set to compare the
// values
fn result_order<T: TextStore>(a: &T::Item, b: &T::Item, query: &str) -> Ordering {
    let a_jaro = result_order_value(query, a.get_text());
    let b_jaro = result_order_value(query, b.get_text());

    if diff(b_jaro, 100) > 10 && diff(a_jaro, 100) > 10 && (b.ord() > 0 || b.ord() > 0) {
        b.ord().cmp(&a.ord())
    } else {
        b_jaro.cmp(&a_jaro)
    }
}

fn result_order_value(query: &str, v: &str) -> u32 {
    (jaro_winkler(&v.get_text().to_lowercase(), &query.to_lowercase()) * 100_f64) as u32
}

#[derive(Clone, Copy)]
pub struct TextSearch<T: TextStore> {
    text_store: T,
}

impl<T: TextStore> TextSearch<T> {
    /// Creates a new [`Serach`] based on searchable data. The input must be sorted and implement
    /// `Ord`
    pub fn new(text_store: T) -> Self {
        Self { text_store }
    }

    /// Returns a vector over all found elements
    pub fn find_all_bin<'a>(&'a self, query: String) -> Vec<&'a T::Item> {
        if query.is_empty() {
            return vec![];
        }

        BinarySearch::new(query, &self.text_store)
            .search()
            .collect()
    }

    /// Returns a vector over all found elements
    pub fn find_all_lev<'a>(&'a self, query: &'a str, len_limit: usize) -> Vec<&'a T::Item> {
        if query.is_empty() {
            return vec![];
        }

        self.find_jaro(query, len_limit).collect()
    }

    pub fn find_jaro<'a>(
        &'a self,
        query: &'a str,
        len_limit: usize,
    ) -> impl Iterator<Item = &'a T::Item> {
        self.jaro_search(query, len_limit).search()
    }

    pub async fn find_jaro_async<'a>(
        &'a self,
        query: &'a str,
        len_limit: usize,
    ) -> Vec<&'a T::Item> {
        let search: AsyncSearch<'_, _> = self.jaro_search(query, len_limit).into();
        search.await
    }

    pub fn find_binary<'a>(&'a self, query: String) -> impl Iterator<Item = &'a T::Item> {
        self.binary_search(query).search()
    }

    fn binary_search<'a>(&'a self, query: String) -> BinarySearch<'a, T> {
        BinarySearch::new(query, &self.text_store)
    }

    fn jaro_search<'a>(&'a self, query: &'a str, len_limit: usize) -> JaroSearch<'a, T> {
        JaroSearch::new(query, &self.text_store, len_limit)
    }
}
