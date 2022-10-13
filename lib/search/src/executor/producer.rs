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
        format_debug_name::<Self>()
    }
}

fn format_debug_name<T: ?Sized>() -> String {
    let mut name = type_name::<T>().to_string();

    // Strip module name
    let start_pos = name
        .char_indices()
        .rev()
        .find(|i| i.1 == ':')
        .map(|i| i.0 + 1)
        .unwrap_or(0);
    name.replace_range(0..start_pos, "");

    name
}
