use crate::{
    executor::{out_builder::OutputBuilder, producer::Producer, searchable::Searchable},
    query::Query,
    sentence::Search,
};
use engine::{
    pushable::{FilteredMaxCounter, Pushable},
    rel_item::RelItem,
};
use types::jotoba::sentences::Sentence;

/// Producer for sentence by seq
pub struct SequenceProducer<'a> {
    query: &'a Query,
}

impl<'a> SequenceProducer<'a> {
    pub fn new(query: &'a Query) -> Self {
        Self { query }
    }

    fn sentence(&self) -> Option<&'static Sentence> {
        let seq = self.query.form.as_sequence()?;
        resources::get().sentences().by_id(*seq)
    }
}

impl<'a> Producer for SequenceProducer<'a> {
    type Target = Search<'a>;

    fn produce(
        &self,
        out: &mut OutputBuilder<
            <Self::Target as Searchable>::Item,
            <Self::Target as Searchable>::ResAdd,
        >,
    ) {
        if let Some(s) = self.sentence() {
            out.push(RelItem::new(s, 0.0));
        }
    }

    fn estimate_to(&self, out: &mut FilteredMaxCounter<<Self::Target as Searchable>::Item>) {
        if let Some(sentence) = self.sentence() {
            out.push(sentence);
        }
    }

    fn should_run(&self, _already_found: usize) -> bool {
        self.query.form.is_sequence()
    }
}
