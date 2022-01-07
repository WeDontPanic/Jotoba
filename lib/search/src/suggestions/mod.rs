mod binary_search;
mod jaro_search;
pub mod store_item;
pub mod text_store;

use self::{jaro_search::AsyncSearch, store_item::Item};
use binary_search::Search as BinarySearch;
use jaro_search::Search as JaroSearch;
use std::{cmp::Ordering, collections::HashMap};
use strsim::jaro_winkler;
use text_store::TextStore;
use utils::diff;

/// Searches for japanese suggestions
pub async fn japanese<'a, T: TextStore>(
    dict: &'a TextSearch<T>,
    query: &'a str,
) -> Vec<&'a T::Item> {
    let mut items: Vec<_> = dict.find_binary(query.to_owned()).take(100).collect();

    let mut cache = HashMap::with_capacity(items.len());
    items.sort_by(|a, b| result_order::<T>(a, b, query, &mut cache));

    items
}

pub async fn generic<'a, T: TextStore>(
    dict: &'a TextSearch<T>,
    query: &'a str,
) -> Vec<&'a T::Item> {
    let mut items: Vec<_> = dict.find_binary(query.to_owned()).take(100).collect();

    if items.len() < 5 {
        let jaro_res = dict.find_jaro_async(query, 5).await;
        items.extend(jaro_res);
    }

    let mut cache = HashMap::with_capacity(items.len());
    items.sort_by(|a, b| result_order::<T>(a, b, query, &mut cache));

    items
}

/// Searches for kanji suggestions by their meanings
pub async fn kanji_meaning<'a, T: TextStore>(
    dict: &'a TextSearch<T>,
    query: &'a str,
) -> Vec<&'a T::Item> {
    let mut items: Vec<_> = dict.find_binary(query.to_owned()).take(100).collect();

    if items.len() < 5 {
        let jaro_res = dict.find_jaro_async(query, 5).await;
        items.extend(jaro_res);
    }

    let mut cache = HashMap::with_capacity(items.len());
    items.sort_by(|a, b| result_order::<T>(a, b, query, &mut cache));

    items
}

// Order by best match against `query`
fn result_order<'a, T: TextStore>(
    a: &'a T::Item,
    b: &'a T::Item,
    query: &str,
    cache: &mut HashMap<&'a str, u32>,
) -> Ordering {
    let a_jaro = *cache
        .entry(a.get_text())
        .or_insert_with(|| result_order_value(query, a.get_text()));
    let b_jaro = *cache
        .entry(b.get_text())
        .or_insert_with(|| result_order_value(query, b.get_text()));

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
