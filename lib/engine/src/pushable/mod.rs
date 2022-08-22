pub mod counter;
pub mod f_max_cnt;
pub mod max_cnt;
pub mod push_mod;

pub use counter::Counter;
pub use f_max_cnt::FilteredMaxCounter;
pub use max_cnt::MaxCounter;
pub use push_mod::PushMod;

use super::relevance::item::RelItem;
use priority_container::StableUniquePrioContainerMax;
use std::hash::Hash;

pub trait Pushable {
    type Item;

    fn push(&mut self, i: Self::Item) -> bool;
}

impl<T> Pushable for StableUniquePrioContainerMax<RelItem<T>>
where
    T: Eq + Hash + Clone,
{
    type Item = RelItem<T>;

    #[inline]
    fn push(&mut self, i: Self::Item) -> bool {
        self.insert(i);
        true
    }
}
