pub mod foreign;
pub mod japanese;
pub mod k_reading;
pub mod regex;
pub mod sequence;
pub mod tag;

/*

use crate::{
    executor::{out_builder::OutputBuilder, producer::Producer, searchable::Searchable},
    query::Query,
    word::Search,
};

pub struct RomajiProducer<'a> {
    query: &'a Query,
}

impl<'a> RomajiProducer<'a> {
    pub fn new(query: &'a Query) -> Self {
        Self { query }
    }
}

impl<'a> Producer for RomajiProducer<'a> {
    type Target = Search<'a>;

    fn produce(
        &self,
        out: &mut OutputBuilder<
            <Self::Target as Searchable>::Item,
            <Self::Target as Searchable>::ResAdd,
        >,
    ) {
        todo!()
    }
}

*/
