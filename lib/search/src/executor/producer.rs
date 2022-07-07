use super::{out_builder::OutputBuilder, searchable::Searchable};
use types::jotoba::search::guess::Guess;

pub trait Producer {
    type Target: Searchable;

    fn produce(
        &self,
        out: &mut OutputBuilder<
            <Self::Target as Searchable>::Item,
            <Self::Target as Searchable>::OutputAdd,
        >,
    );

    fn should_run(&self, _already_found: usize) -> bool {
        true
    }

    fn estimate(&self) -> Option<Guess> {
        None
    }
}
