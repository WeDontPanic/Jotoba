use crate::{
    engine::{words::native, SearchTask},
    query::Query,
    word::order,
};

/// Helper for creating SearchTask for foreign queries
pub struct NativeSearch<'a> {
    _query: &'a Query,
    query_str: &'a str,
}

impl<'a> NativeSearch<'a> {
    pub(crate) fn new(query: &'a Query, query_str: &'a str) -> Self {
        Self {
            _query: query,
            query_str,
        }
    }

    pub fn task(&self) -> SearchTask<native::Engine> {
        let mut task: SearchTask<native::Engine> =
            SearchTask::new(self.query_str).threshold(0.4f32);

        let original_query = self.query_str.to_string();
        task.with_custom_order(move |item| {
            order::japanese_search_order(item, Some(&original_query))
        });

        // TODO: apply filter

        task
    }
}
