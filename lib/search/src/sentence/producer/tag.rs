use crate::{
    executor::{out_builder::OutputBuilder, producer::Producer, searchable::Searchable},
    query::{Query, Tag},
    sentence::Search,
};
use engine::{
    pushable::FilteredMaxCounter,
    pushable::{PushMod, Pushable},
    rel_item::RelItem,
};
use types::jotoba::sentences::Sentence;

/// Producer for Tags
pub struct TagProducer<'a> {
    query: &'a Query,
}

impl<'a> TagProducer<'a> {
    pub fn new(query: &'a Query) -> Self {
        Self { query }
    }

    fn find_to<P>(&self, out: &mut P)
    where
        P: Pushable<Item = RelItem<&'static Sentence>>,
    {
        let tag = self
            .query
            .tags
            .iter()
            .filter(|i| i.is_jlpt() || i.is_sentence_tag())
            .find(|i| i.is_producer())
            .unwrap();
        self.push_tag(tag, out);
    }

    pub fn push_tag<P>(&self, tag: &Tag, out: &mut P)
    where
        P: Pushable<Item = RelItem<&'static Sentence>>,
    {
        let s_res = resources::get().sentences();

        match tag {
            Tag::SentenceTag(sentence_tag) => self.push_iter(s_res.by_tag(sentence_tag), out),
            Tag::Jlpt(jlpt) => self.push_iter(s_res.by_jlpt(*jlpt), out),
            _ => (),
        }
    }

    fn push_iter<P, I>(&self, iter: I, out: &mut P)
    where
        P: Pushable<Item = RelItem<&'static Sentence>>,
        I: Iterator<Item = &'static Sentence>,
    {
        let mut c = 0;
        for w in iter {
            let item = RelItem::new(w, c as f32);
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

    fn estimate_to(&self, out: &mut FilteredMaxCounter<<Self::Target as Searchable>::Item>) {
        let mut m = PushMod::new(out, |i: RelItem<&Sentence>| i.item);
        self.find_to(&mut m);
    }

    fn should_run(&self, _already_found: usize) -> bool {
        self.query.query_str.is_empty()
            && self
                .query
                .tags
                .iter()
                // Only run for jjlpt and sentence tags
                .filter(|i| i.is_jlpt() || i.is_sentence_tag())
                .any(|i| i.is_producer())
    }
}
