use std::ops::Deref;

/// The final result of a search
#[derive(Clone)]
pub struct SearchResult<T, O = ()> {
    pub items: Vec<T>,
    pub total: usize,
    pub other_data: O,
}

impl<T> SearchResult<T, ()> {
    /// Creates a new SearchResult from a vec
    pub fn from_vec(items: Vec<T>) -> Self {
        let total = items.len();
        Self {
            items,
            total,
            other_data: (),
        }
    }

    /// Creates a new search result
    pub fn new(items: Vec<T>, total: usize) -> Self {
        Self {
            items,
            total,
            other_data: (),
        }
    }
}

impl<T, O> SearchResult<T, O> {
    /// Creates a new search result
    pub fn with_other_data(items: Vec<T>, total: usize, other_data: O) -> Self {
        Self {
            items,
            total,
            other_data,
        }
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.items.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.items.iter()
    }
}

impl<T, O: Default> SearchResult<T, O> {
    /// Creates a new search result
    pub fn with_other_default(items: Vec<T>, total: usize) -> Self {
        Self {
            items,
            total,
            other_data: O::default(),
        }
    }
}

impl<T, O> Default for SearchResult<T, O>
where
    O: Default,
{
    #[inline]
    fn default() -> Self {
        Self {
            items: vec![],
            total: 0,
            other_data: O::default(),
        }
    }
}

impl<T, O> Deref for SearchResult<T, O> {
    type Target = O;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.other_data
    }
}
