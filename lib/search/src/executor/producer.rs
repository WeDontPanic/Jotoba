use std::any::type_name;

use super::{out_builder::OutputBuilder, searchable::Searchable};
use types::jotoba::search::guess::Guess;

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

    fn estimate(&self) -> Option<Guess> {
        None
    }

    fn name(&self) -> String {
        type_name::<Self>().to_string()
    }
}
