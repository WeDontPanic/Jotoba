use types::jotoba::words::Word;

use crate::{
    engine::{
        result_item::ResultItem,
        search_task::{
            cpushable::FilteredMaxCounter,
            pushable::{PushMod, Pushable},
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

    fn find_to<P>(&self, out: &mut P)
    where
        P: Pushable<Item = ResultItem<&'static Word>>,
    {
        let producer_tag = self.query.tags.iter().find(|i| i.is_producer()).unwrap();
        self.find_words(out, producer_tag);
    }

    fn find_words<P>(&self, out: &mut P, tag: &Tag)
    where
        P: Pushable<Item = ResultItem<&'static Word>>,
    {
        match tag {
            Tag::PartOfSpeech(pos) => self.push_generic(out, |w| w.has_pos(&[*pos])),
            Tag::Misc(m) => self.push_generic(out, |w| w.has_misc(m)),
            Tag::Jlpt(jlpt) => self.push_jlpt(out, *jlpt),
            Tag::IrregularIruEru => self.push_irr_ichidan(out),
            _ => (),
        }
    }

    fn push_jlpt<P>(&self, out: &mut P, jlpt: u8)
    where
        P: Pushable<Item = ResultItem<&'static Word>>,
    {
        for w in resources::get().words().by_jlpt(jlpt) {
            let len = w.get_reading().len();
            let rel = 1000usize.saturating_sub(len);
            out.push(ResultItem::new(w, rel))
        }
    }

    fn push_irr_ichidan<P>(&self, out: &mut P)
    where
        P: Pushable<Item = ResultItem<&'static Word>>,
    {
        for w in resources::get().words().irregular_ichidan() {
            out.push(ResultItem::new(w, 0));
        }
    }

    fn push_generic<P, F>(&self, out: &mut P, filter: F)
    where
        P: Pushable<Item = ResultItem<&'static Word>>,
        F: Fn(&Word) -> bool,
    {
        for w in resources::get().words().iter() {
            if !filter(w) {
                continue;
            }

            let score = generic_order(w);
            out.push(ResultItem::new(w, score));
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
        self.query.query_str.is_empty() && self.query.tags.iter().any(|i| i.is_producer())
    }

    fn estimate_to(&self, out: &mut FilteredMaxCounter<<Self::Target as Searchable>::Item>) {
        let mut mid = PushMod::new(out, |i: ResultItem<&Word>| i.item);
        self.find_to(&mut mid);
    }
}

fn generic_order(word: &Word) -> usize {
    let mut score = 0usize;

    if word.is_common() {
        score += 10;
    }

    if let Some(jlpt) = word.jlpt_lvl {
        score += 3 + jlpt as usize;
    }

    if word.has_collocations() {
        score += 1;
    }

    if word.has_pitch() {
        score += 1;
    }

    score
}
