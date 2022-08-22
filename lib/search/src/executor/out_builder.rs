use engine::{pushable::Pushable, relevance::item::RelItem};
use priority_container::StableUniquePrioContainerMax;
use std::hash::Hash;

pub struct OutputBuilder<'a, I, OA> {
    pub(crate) p: StableUniquePrioContainerMax<RelItem<I>>,
    pub(crate) filter: Box<dyn Fn(&I) -> bool + 'a>,
    pub(crate) output_add: OA,
}

impl<'a, I: Eq + Hash + Clone, OA: Default> OutputBuilder<'a, I, OA> {
    pub(crate) fn new<F: Fn(&I) -> bool + 'a>(filter: F, len: usize) -> Self {
        assert!(len > 0);
        let p = StableUniquePrioContainerMax::new(len);
        let filter = Box::new(filter);
        let output_add = OA::default();
        Self {
            p,
            filter,
            output_add,
        }
    }

    /// Pushes an element into the output and  returns `true` if it was not filtered out
    #[inline]
    pub fn push(&mut self, item: RelItem<I>) -> bool {
        if !(self.filter)(&item.item) {
            self.p.insert(item);
            return true;
        }

        false
    }
}

impl<'a, I: Eq + Hash + Clone, OA: Default> Pushable for OutputBuilder<'a, I, OA> {
    type Item = RelItem<I>;

    /// Pushes an element into the output and  returns `true` if it was not filtered out
    #[inline]
    fn push(&mut self, i: Self::Item) -> bool {
        self.push(i)
    }
}
