use std::hash::Hash;

use priority_container::StableUniquePrioContainerMax;

use crate::engine::result_item::ResultItem;

pub trait Pushable {
    type Item;

    fn push(&mut self, i: Self::Item);
}

impl<T: PartialEq + Hash + Eq + Clone> Pushable for StableUniquePrioContainerMax<ResultItem<T>> {
    type Item = ResultItem<T>;

    #[inline]
    fn push(&mut self, i: Self::Item) {
        self.insert(i);
    }
}
