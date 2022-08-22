use crate::{
    executor::{out_builder::OutputBuilder, producer::Producer, searchable::Searchable},
    query::Query,
    word::Search,
};
use engine::{pushable::FilteredMaxCounter, pushable::Pushable, rel_item::RelItem};
use types::jotoba::words::Word;

/// Producer for a Word by its sequence id
pub struct SeqProducer<'a> {
    query: &'a Query,
}

impl<'a> SeqProducer<'a> {
    pub fn new(query: &'a Query) -> Self {
        Self { query }
    }

    // Find the word
    pub fn word(&self) -> Option<&'static Word> {
        let seq = *self.query.form.as_sequence()?;
        resources::get().words().by_sequence(seq)
    }
}

impl<'a> Producer for SeqProducer<'a> {
    type Target = Search<'a>;

    fn produce(
        &self,
        out: &mut OutputBuilder<
            <Self::Target as Searchable>::Item,
            <Self::Target as Searchable>::ResAdd,
        >,
    ) {
        if let Some(word) = self.word() {
            out.push(RelItem::new(word, 0.0));
        }
    }

    fn should_run(&self, _already_found: usize) -> bool {
        self.query.form.is_sequence()
    }

    fn estimate_to(&self, out: &mut FilteredMaxCounter<<Self::Target as Searchable>::Item>) {
        if let Some(word) = self.word() {
            out.push(word);
        }
    }
}
