use crate::{
    engine::result_item::ResultItem,
    executor::{out_builder::OutputBuilder, producer::Producer, searchable::Searchable},
    query::Query,
    sentence::Search,
};
use types::jotoba::{
    search::guess::{Guess, GuessType},
    sentences::Sentence,
};

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
            out.push(ResultItem::new(s, 0));
        }
    }

    fn estimate(&self) -> Option<types::jotoba::search::guess::Guess> {
        let count = self.sentence().is_some() as u32;
        Some(Guess::new(count, GuessType::Accurate))
    }

    fn should_run(&self, _already_found: usize) -> bool {
        self.query.form.is_sequence()
    }
}
