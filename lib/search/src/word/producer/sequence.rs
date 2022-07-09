use types::jotoba::{
    search::guess::{Guess, GuessType},
    words::Word,
};

use crate::{
    engine::result_item::ResultItem,
    executor::{out_builder::OutputBuilder, producer::Producer, searchable::Searchable},
    query::Query,
    word::Search,
};

pub struct SeqProducer<'a> {
    query: &'a Query,
}

impl<'a> SeqProducer<'a> {
    pub fn new(query: &'a Query) -> Self {
        Self { query }
    }

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
            <Self::Target as Searchable>::OutputAdd,
        >,
    ) {
        if let Some(word) = self.word() {
            out.push(ResultItem::new(word, 0));
        }
    }

    fn should_run(&self, _already_found: usize) -> bool {
        self.query.form.is_sequence()
    }

    fn estimate(&self) -> Option<Guess> {
        let count = self.word().map_or(1, |_| 0);
        Some(Guess::new(count, GuessType::Accurate))
    }
}
