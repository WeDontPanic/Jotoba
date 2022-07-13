use types::jotoba::words::Word;

use crate::{
    engine::{
        result_item::ResultItem,
        search_task::{
            cpushable::{CPushable, FilteredMaxCounter},
            pushable::PushMod,
        },
    },
    executor::{out_builder::OutputBuilder, producer::Producer, searchable::Searchable},
    query::{Query, Tag},
    word::Search,
};

pub struct TagProducer<'a> {
    query: &'a Query,
}

impl<'a> TagProducer<'a> {
    pub fn new(query: &'a Query) -> Self {
        Self { query }
    }

    fn get_producer_tag(&self) -> Option<&Tag> {
        self.query
            .tags
            .iter()
            .find(|i| i.is_producer() && !i.is_sentence_tag())
    }

    fn find_to<P>(&self, out: &mut P)
    where
        P: CPushable<Item = ResultItem<&'static Word>>,
    {
        // Find first producer tag. All other tags are treated as filter
        let producer_tag = self.get_producer_tag().unwrap();
        self.find_words(out, producer_tag);
    }

    fn find_words<P>(&self, out: &mut P, tag: &Tag)
    where
        P: CPushable<Item = ResultItem<&'static Word>>,
    {
        let words = resources::get().words();
        match tag {
            Tag::PartOfSpeech(pos) => self.push_iter(words.by_pos_simple(*pos), out),
            Tag::Misc(m) => self.push_iter(words.by_misc(*m), out),
            Tag::Jlpt(jlpt) => self.push_iter(words.by_jlpt(*jlpt), out),
            Tag::IrregularIruEru => self.push_iter(words.irregular_ichidan(), out),
            _ => (),
        }
    }

    fn push_iter<P, I>(&self, iter: I, out: &mut P)
    where
        P: CPushable<Item = ResultItem<&'static Word>>,
        I: Iterator<Item = &'static Word> + DoubleEndedIterator,
    {
        let mut c = 0;
        for w in iter.rev() {
            let item = ResultItem::new(w, 1000 - c);
            if out.push(item) {
                c += 1;
                if c >= 1000 {
                    break;
                }
            }
        }
    }
}

impl<'a> Producer for TagProducer<'a> {
    type Target = Search<'a>;

    fn produce(
        &self,
        out: &mut OutputBuilder<
            <Self::Target as Searchable>::Item,
            <Self::Target as Searchable>::ResAdd,
        >,
    ) {
        self.find_to(out);
    }

    fn should_run(&self, _already_found: usize) -> bool {
        // Only run this producer if there is no query (except tags) and there are tags which can produce output
        self.query.query_str.is_empty() && self.get_producer_tag().is_some()
    }

    fn estimate_to(&self, out: &mut FilteredMaxCounter<<Self::Target as Searchable>::Item>) {
        let mut mid = PushMod::new(out, |i: ResultItem<&Word>| i.item);
        self.find_to(&mut mid);
    }
}
