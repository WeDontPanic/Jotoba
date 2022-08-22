use super::{out_builder::OutputBuilder, searchable::Searchable};
use engine::pushable::FilteredMaxCounter;
use std::any::type_name;

pub trait Producer {
    type Target: Searchable;

    fn produce(
        &self,
        out: &mut OutputBuilder<
            <Self::Target as Searchable>::Item,
            <Self::Target as Searchable>::ResAdd,
        >,
    );

    fn should_run(&self, _already_found: usize) -> bool {
        true
    }

    fn estimate_to(&self, _out: &mut FilteredMaxCounter<<Self::Target as Searchable>::Item>) {}

    fn name(&self) -> String {
        type_name::<Self>().to_string()
    }
}
