use super::query::Query;

pub struct SearchOrder<'a, 'parser> {
    pub query: &'a Query,
    pub morpheme: &'a Option<WordItem<'parser, 'a>>,
}

impl<'a, 'parser> SearchOrder<'a, 'parser> {
    #[inline]
    pub fn new(query: &'a Query, morpheme: &'a Option<WordItem<'parser, 'a>>) -> Self {
        SearchOrder { query, morpheme }
    }

    #[inline]
    pub fn sort<U, T>(&self, vec: &mut Vec<U>, order_fn: T)
    where
        T: Fn(&U, &SearchOrder) -> usize,
    {
        vec.sort_by(|a, b| order_fn(a, &self).cmp(&order_fn(b, &self)).reverse())
    }
}
