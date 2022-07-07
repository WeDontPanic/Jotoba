use types::jotoba::{
    names::Name,
    search::guess::{Guess, GuessType},
};

use crate::{
    engine::result_item::ResultItem,
    executor::{out_builder::OutputBuilder, producer::Producer, searchable::Searchable},
    name::Search,
    query::{Form, Query},
};

pub struct SeqProducer<'a> {
    query: &'a Query,
}

impl<'a> SeqProducer<'a> {
    pub fn new(query: &'a Query) -> Self {
        Self { query }
    }

    fn name(&self) -> Option<&'static Name> {
        if let Form::Sequence(seq) = self.query.form {
            return resources::get().names().by_sequence(seq);
        }
        None
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
        if let Some(name) = self.name() {
            out.push(ResultItem::new(name, 0));
        }
    }

    fn estimate(&self) -> Option<types::jotoba::search::guess::Guess> {
        let len = self.name().map_or(1, |_| 0);
        Some(Guess::new(len, GuessType::Accurate))
    }

    fn should_run(&self, _already_found: usize) -> bool {
        self.query.form.is_sequence()
    }
}
