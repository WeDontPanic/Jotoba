use types::jotoba::languages::Language;
use vector_space_model2::Vector;

#[derive(Clone, Copy)]
pub struct SortItem<'a, 'query, T> {
    item: &'a T,
    rel: f32,
    query: &'query str,
    language: Option<Language>,
    query_vec: &'query Vector,
    item_vec: &'query Vector,
}

impl<'a, 'query, T> SortItem<'a, 'query, T> {
    #[inline]
    pub fn new(
        item: &'a T,
        rel: f32,
        query: &'query str,
        language: Option<Language>,
        query_vec: &'query Vector,
        item_vec: &'query Vector,
    ) -> Self {
        Self {
            item,
            rel,
            query,
            language,
            query_vec,
            item_vec,
        }
    }

    #[inline]
    pub fn item(&self) -> &T {
        self.item
    }

    #[inline]
    pub fn rel(&self) -> f32 {
        self.rel
    }

    #[inline]
    pub fn query(&self) -> &str {
        self.query
    }

    #[inline]
    pub fn language(&self) -> Option<Language> {
        self.language
    }

    #[inline]
    pub fn item_vec(&self) -> &Vector {
        self.item_vec
    }

    #[inline]
    pub fn query_vec(&self) -> &Vector {
        self.query_vec
    }

    #[inline]
    pub fn vec_simiarity(&self) -> f32 {
        self.query_vec.similarity(self.item_vec)
    }
}
