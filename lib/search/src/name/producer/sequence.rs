use crate::{
    engine::{
        result_item::ResultItem,
        search_task::cpushable::{CPushable, FilteredMaxCounter},
    },
    executor::{out_builder::OutputBuilder, producer::Producer, searchable::Searchable},
    name::Search,
    query::Query,
};
use types::jotoba::names::Name;

pub struct SeqProducer<'a> {
    query: &'a Query,
}

impl<'a> SeqProducer<'a> {
    pub fn new(query: &'a Query) -> Self {
        Self { query }
    }

    fn name(&self) -> Option<&'static Name> {
        let seq = *self.query.form.as_sequence()?;
        resources::get().names().by_sequence(seq)
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
        if let Some(name) = self.name() {
            out.push(ResultItem::new(name, 0));
        }
    }

    fn estimate_to(&self, out: &mut FilteredMaxCounter<<Self::Target as Searchable>::Item>) {
        if let Some(name) = self.name() {
            out.push(name);
        }
    }

    fn should_run(&self, _already_found: usize) -> bool {
        self.query.form.is_sequence()
    }
}
