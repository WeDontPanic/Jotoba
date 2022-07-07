mod producer;
pub mod result;

use super::query::Query;
use crate::{
    executor::{out_builder::OutputBuilder, producer::Producer, searchable::Searchable},
    query::{QueryLang, Tag},
};
use producer::{sentences::SentenceProducer, sequence::SequenceProducer, tag::TagProducer};
use result::ResData;
use types::jotoba::{languages::Language, sentences::Sentence};

pub struct Search<'a> {
    query: &'a Query,
    producer: Vec<Box<dyn Producer<Target = Self> + 'a>>,
}

impl<'a> Search<'a> {
    pub fn new(query: &'a Query) -> Self {
        let sent_producer = SentenceProducer::new(query, query.q_lang == QueryLang::Japanese);
        let seq_producer = SequenceProducer::new(query);
        let tag_producer = TagProducer::new(query);

        let producer: Vec<Box<dyn Producer<Target = Self>>> = vec![
            Box::new(sent_producer),
            Box::new(seq_producer),
            Box::new(tag_producer),
        ];

        Self { query, producer }
    }
}

impl<'a> Searchable for Search<'a> {
    type OutputAdd = ResData;
    type OutputItem = result::Sentence;
    type Item = &'static Sentence;

    fn get_producer<'s>(&'s self) -> &Vec<Box<dyn Producer<Target = Self> + 's>> {
        &self.producer
    }

    fn mod_output(&self, out: &mut OutputBuilder<Self::Item, Self::OutputAdd>) {
        out.output_add = ResData::new(self.query.has_tag(Tag::Hidden));
    }

    #[inline]
    fn to_output_item(&self, item: Self::Item) -> Self::OutputItem {
        let lang = self.query.settings.language();
        let show_english = self.query.settings.show_english;
        map_sentence_to_item(item, lang, show_english).unwrap()
    }

    fn get_query(&self) -> &Query {
        self.query
    }
}

pub fn map_sentence_to_item(
    sentence: &Sentence,
    lang: Language,
    show_english: bool,
) -> Option<result::Sentence> {
    result::Sentence::from_m_sentence(sentence.clone(), lang, show_english)
}
