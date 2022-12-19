use sparse_vec::{SpVec32, VecExt};
use types::jotoba::language::Language;

/// Item to sort stuff
#[derive(Debug)]
pub struct SortData<'item, 'query, T, I, Q> {
    out_item: &'item T,
    index_item: &'item I,
    rel: f32,
    query_str: &'query str,
    query: &'query Q,
    language: Option<Language>,
    threshold: Option<f32>,
}

impl<'item, 'query, T, I, Q> SortData<'item, 'query, T, I, Q> {
    #[inline]
    pub fn new(
        out_item: &'item T,
        index_item: &'item I,
        rel: f32,
        query: &'query Q,
        query_str: &'query str,
        language: Option<Language>,
        threshold: Option<f32>,
    ) -> Self {
        Self {
            out_item,
            index_item,
            rel,
            query_str,
            query,
            language,
            threshold,
        }
    }

    #[inline]
    pub fn item(&self) -> &T {
        self.out_item
    }

    #[inline]
    pub fn rel(&self) -> f32 {
        self.rel
    }

    #[inline]
    pub fn query_str(&self) -> &str {
        self.query_str
    }

    #[inline]
    pub fn language(&self) -> Option<Language> {
        self.language
    }

    #[inline]
    pub fn query(&self) -> &'query Q {
        self.query
    }

    #[inline]
    pub fn index_item(&self) -> &I {
        self.index_item
    }

    #[inline]
    pub fn threshold(&self) -> Option<f32> {
        self.threshold
    }
}

impl<'item, 'query, T, I> SortData<'item, 'query, T, I, SpVec32>
where
    I: AsRef<SpVec32>,
{
    #[inline]
    pub fn vec_similarity(&self) -> f32 {
        self.query.cosine(self.index_item.as_ref())
    }
}
