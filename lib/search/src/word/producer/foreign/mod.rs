pub mod romaji;

use crate::{
    executor::{out_builder::OutputBuilder, producer::Producer, searchable::Searchable},
    query::Query,
    word::Search,
};

pub struct ForeignProducer<'a> {
    query: &'a Query,
}

impl<'a> ForeignProducer<'a> {
    pub fn new(query: &'a Query) -> Self {
        Self { query }
    }
}

impl<'a> Producer for ForeignProducer<'a> {
    type Target = Search<'a>;

    fn produce(
        &self,
        out: &mut OutputBuilder<
            <Self::Target as Searchable>::Item,
            <Self::Target as Searchable>::ResAdd,
        >,
    ) {
        //
    }

    fn should_run(&self, already_found: usize) -> bool {
        true
    }
}
