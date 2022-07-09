mod producer;

use crate::{
    executor::{producer::Producer, searchable::Searchable},
    query::Query,
};
use producer::{kanji_reading::KreadingProducer, names::NameProducer, sequence::SeqProducer};
use types::jotoba::names::Name;

pub struct Search<'a> {
    query: &'a Query,
    producer: Vec<Box<dyn Producer<Target = Self> + 'a>>,
}

impl<'a> Search<'a> {
    pub fn new(query: &'a Query) -> Self {
        let mut producer: Vec<Box<dyn Producer<Target = Self>>> = vec![];
        producer.push(Box::new(SeqProducer::new(query)));
        producer.push(Box::new(KreadingProducer::new(query)));
        producer.push(Box::new(NameProducer::new(query)));
        Self { query, producer }
    }
}

impl<'a> Searchable for Search<'a> {
    type Item = &'static Name;
    type OutItem = &'static Name;
    type ResAdd = ();

    #[inline]
    fn to_output_item(&self, item: Self::Item) -> Self::OutItem {
        item
    }

    fn get_producer<'s>(&'s self) -> &Vec<Box<dyn Producer<Target = Self> + 's>> {
        &self.producer
    }

    fn get_query(&self) -> &Query {
        self.query
    }
}
