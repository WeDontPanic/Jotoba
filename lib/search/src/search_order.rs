use super::query::Query;

#[cfg(feature = "tokenizer")]
use japanese::jp_parsing::WordItem;

#[cfg(feature = "tokenizer")]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SearchOrder<'a, 'parser> {
    pub query: &'a Query,
    pub morpheme: &'a Option<WordItem<'parser, 'a>>,
}

#[cfg(not(feature = "tokenizer"))]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SearchOrder<'a> {
    pub query: &'a Query,
}

#[cfg(feature = "tokenizer")]
impl<'a, 'parser> SearchOrder<'a, 'parser> {
    pub fn new(query: &'a Query, morpheme: &'a Option<WordItem<'parser, 'a>>) -> Self {
        SearchOrder { query, morpheme }
    }

    pub fn sort<U, T>(&self, vec: &mut Vec<U>, order_fn: T)
    where
        T: Fn(&U, &SearchOrder) -> usize,
    {
        vec.sort_by(|a, b| order_fn(a, &self).cmp(&order_fn(b, &self)).reverse())
    }
}

#[cfg(not(feature = "tokenizer"))]
impl<'a, 'parser> SearchOrder<'a> {
    pub fn new(query: &'a Query, _morpheme: &Option<u32>) -> Self {
        SearchOrder { query }
    }

    pub fn sort<U, T>(&self, vec: &mut Vec<U>, order_fn: T)
    where
        T: Fn(&U, &SearchOrder) -> usize,
    {
        vec.sort_by(|a, b| order_fn(a, &self).cmp(&order_fn(b, &self)).reverse())
    }
}
