use crate::{
    executor::{out_builder::OutputBuilder, producer::Producer, searchable::Searchable},
    query::Query,
    word::Search,
};

use engine::pushable::FilteredMaxCounter;
use japanese_number_parser::JapaneseNumberFormatter;
use jp_utils::JapaneseExt;
use log::debug;

/// Produces a number if the query is a Japanese number
pub struct NumberProducer<'a> {
    query: &'a Query,
}

impl<'a> NumberProducer<'a> {
    #[inline]
    pub fn new(query: &'a Query) -> Self {
        Self { query }
    }
}

impl<'a> Producer for NumberProducer<'a> {
    type Target = Search<'a>;

    fn produce(
        &self,
        out: &mut OutputBuilder<
            <Self::Target as Searchable>::Item,
            <Self::Target as Searchable>::ResAdd,
        >,
    ) {
        let query = &self.query.query_str;
        if let Some(number) = JapaneseNumberFormatter::new().format(&query) {
            debug!("Found number: {number:?}");
            out.output_add.number = Some(number);
        }
    }

    fn estimate_to(&self, _out: &mut FilteredMaxCounter<<Self::Target as Searchable>::Item>) {}

    fn should_run(&self, _already_found: usize) -> bool {
        let query_str = &self.query.query_str;

        !query_str.is_empty()
        // Don't parse if query is a regular number
            && query_str
                .to_halfwidth()
                .parse::<usize>()
                .is_err()
    }
}
