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
            .filter(|i| i.is_producer() && !i.is_sentence_tag())
            // Use tag with fewest items that it'll produce to reduce the amount of items that have to be filtered
            .map(|i| (self.tag_len(i).unwrap_or(usize::MAX), i))
            .min_by_key(|i| i.0)
            .map(|i| i.1)
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

    /// Returns the amount of words a given tag has assigned/indexed
    #[inline]
    fn tag_len(&self, tag: &Tag) -> Option<usize> {
        let w_retr = resources::get().words();
        match tag {
            Tag::PartOfSpeech(p) => w_retr.pos_simple_len(p),
            Tag::Misc(m) => w_retr.misc_len(m),
            Tag::Jlpt(j) => w_retr.jlpt_len(*j),
            Tag::IrregularIruEru => todo!(),
            _ => None,
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
