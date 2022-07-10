pub mod kanji;
pub mod order;
pub mod producer;
pub mod result;

use crate::{
    executor::{out_builder::OutputBuilder, producer::Producer, searchable::Searchable},
    query::Query,
};
use types::jotoba::words::{adjust_language, Word};

use producer::{
    foreign::{romaji::RomajiProducer, ForeignProducer},
    japanese::NativeProducer,
    k_reading::KReadingProducer,
    regex::RegexProducer,
    sequence::SeqProducer,
    tag::TagProducer,
};

use self::producer::japanese::sentence_reader::SReaderProducer;

/// Word search
pub struct Search<'a> {
    query: &'a Query,
    producer: Vec<Box<dyn Producer<Target = Self> + 'a>>,
}

impl<'a> Search<'a> {
    pub fn new(query: &'a Query) -> Self {
        let producer: Vec<Box<dyn Producer<Target = Self>>> = vec![
            Box::new(KReadingProducer::new(query)),
            Box::new(TagProducer::new(query)),
            Box::new(SeqProducer::new(query)),
            Box::new(RegexProducer::new(query)),
            Box::new(RomajiProducer::new(query)),
            Box::new(SReaderProducer::new(query)),
            Box::new(NativeProducer::new(query)),
            Box::new(ForeignProducer::new(query)),
        ];

        Self { query, producer }
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
        adjust_language(
            &mut item,
            self.query.get_search_lang(),
            self.query.settings.show_english(),
        );
        item
    }

    fn filter(&self, word: &Self::Item) -> bool {
        // TODO: apply filters: tags, "must-contain"

        // Filter word if doesn't have proper language
        !word.has_language(
            self.query.get_search_lang(),
            self.query.settings.show_english(),
        )
    }
}
