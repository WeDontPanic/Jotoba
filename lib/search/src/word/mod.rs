pub mod filter;
pub mod kanji;
pub mod order;
pub mod producer;
pub mod result;

use crate::{
    executor::{out_builder::OutputBuilder, producer::Producer, searchable::Searchable},
    query::Query,
};
use types::jotoba::words::Word;

use filter::WordFilter;
use producer::{
    foreign::{romaji::RomajiProducer, ForeignProducer},
    japanese::{number::NumberProducer, sentence_reader::SReaderProducer, NativeProducer},
    k_reading::KReadingProducer,
    regex::RegexProducer,
    sequence::SeqProducer,
    tag::TagProducer,
};

/// Word search
pub struct Search<'a> {
    query: &'a Query,
    producer: Vec<Box<dyn Producer<Target = Self> + 'a>>,
    filter: WordFilter,
}

impl<'a> Search<'a> {
    pub fn new(query: &'a Query) -> Self {
        let producer: Vec<Box<dyn Producer<Target = Self>>> = vec![
            Box::new(KReadingProducer::new(query)),
            Box::new(TagProducer::new(query)),
            Box::new(SeqProducer::new(query)),
            Box::new(RegexProducer::new(query)),
            Box::new(SReaderProducer::new(query)),
            Box::new(NativeProducer::new(query)),
            Box::new(ForeignProducer::new(query)),
            Box::new(RomajiProducer::new(query)),
            Box::new(NumberProducer::new(query)),
        ];

        let filter = WordFilter::new(query.clone());
        Self {
            query,
            producer,
            filter,
        }
    }
}

impl<'a> Searchable for Search<'a> {
    type Item = &'static Word;
    type OutItem = Word;
    type ResAdd = result::AddResData;

    fn get_producer<'s>(&'s self) -> &Vec<Box<dyn Producer<Target = Self> + 's>> {
        &self.producer
    }

    fn get_query(&self) -> &Query {
        self.query
    }

    fn mod_output(&self, out: &mut OutputBuilder<Self::Item, Self::ResAdd>) {
        if out.output_add.raw_query.is_empty() {
            out.output_add.raw_query = self.query.raw_query.clone();
        }
    }

    #[inline]
    fn to_output_item(&self, item: Self::Item) -> Self::OutItem {
        let mut item = item.to_owned();
        item.adjust_language(self.query.lang_param());
        item
    }

    #[inline]
    fn filter(&self, word: &Self::Item) -> bool {
        self.filter.filter_word(*word)
    }

    #[inline]
    fn max_top_dist(&self) -> Option<f32> {
        if !max_top_dist_filter(&self.query) {
            return None;
        }
        //Some(2.0)
        None
    }
}

#[inline]
fn max_top_dist_filter(query: &Query) -> bool {
    !query.is_regex() && query.form.is_normal()
}
