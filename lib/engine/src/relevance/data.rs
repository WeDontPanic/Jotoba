use types::jotoba::languages::Language;
use vector_space_model2::Vector;

/// Item to sort stuff
pub struct SortData<'item, 'query, T, I, Q> {
    out_item: &'item T,
    index_item: &'item I,
    rel: f32,
    query_str: &'query str,
    query: &'query Q,
    language: Option<Language>,
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
    ) -> Self {
        Self {
            out_item,
            index_item,
            rel,
            query_str,
            query,
            language,
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
}

impl<'item, 'query, T> SortData<'item, 'query, T, Vector, Vector> {
    #[inline]
    pub fn vec_similarity(&self) -> f32 {
        self.query.similarity(&self.index_item)
    }
}
