pub mod order;
mod producer;
pub mod result;

use super::query::Query;
use crate::{
    executor::{out_builder::OutputBuilder, producer::Producer, searchable::Searchable},
    query::Tag,
};
use producer::{
    foreign::ForeignProducer, native::NativeProducer, sequence::SequenceProducer, tag::TagProducer,
};
use result::ResData;
use types::jotoba::{languages::Language, sentences::Sentence};

pub struct Search<'a> {
    query: &'a Query,
    producer: Vec<Box<dyn Producer<Target = Self> + 'a>>,
}

impl<'a> Search<'a> {
    pub fn new(query: &'a Query) -> Self {
        let mut producer: Vec<Box<dyn Producer<Target = Self>>> = vec![
            Box::new(SequenceProducer::new(query)),
            Box::new(ForeignProducer::new(query, query.lang())),
            Box::new(TagProducer::new(query)),
            Box::new(NativeProducer::new(query, query.lang())),
        ];

        if query.lang() != Language::English && query.show_english() {
            producer.push(Box::new(ForeignProducer::new(query, Language::English)));
            producer.push(Box::new(NativeProducer::new(query, Language::English)));
        }

        Self { query, producer }
    }
}

impl<'a> Searchable for Search<'a> {
    type ResAdd = ResData;
    type OutItem = result::Sentence;
    type Item = &'static Sentence;

    fn get_producer<'s>(&'s self) -> &Vec<Box<dyn Producer<Target = Self> + 's>> {
        &self.producer
    }

    fn mod_output(&self, out: &mut OutputBuilder<Self::Item, Self::ResAdd>) {
        out.output_add = ResData::new(self.query.has_tag(Tag::Hidden));
    }

    #[inline]
    fn to_output_item(&self, item: Self::Item) -> Self::OutItem {
        let lang = self.query.settings.language();
        let show_english = self.query.settings.show_english;
        result::Sentence::from_m_sentence(item, lang, show_english).unwrap()
    }

    fn get_query(&self) -> &Query {
        self.query
    }

    #[inline]
    fn filter(&self, item: &Self::Item) -> bool {
        !producer::filter::filter_sentence(self.query, item)
    }

    #[inline]
    fn max_top_dist(&self) -> Option<f32> {
        Some(0.9)
        //None
    }
}
