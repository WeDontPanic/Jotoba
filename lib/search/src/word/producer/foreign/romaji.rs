use japanese::{guessing::could_be_romaji, JapaneseExt};

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

    fn romaji_query(&self) -> String {
        self.query.query_str.to_hiragana()
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

    fn should_run(&self, already_found: usize) -> bool {
        already_found < 100 && could_be_romaji(&self.query.query_str)
    }
}
