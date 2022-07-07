use crate::engine::{result_item::ResultItem, search_task::pushable::Pushable};
use priority_container::StableUniquePrioContainerMax;
use std::hash::Hash;

pub struct OutputBuilder<'a, I, OA> {
    pub(crate) p: StableUniquePrioContainerMax<ResultItem<I>>,
    pub(crate) filter: Box<dyn Fn(&I) -> bool + 'a>,
    pub(crate) output_add: OA,
}

impl<'a, I: Eq + Hash + Clone, OA: Default> OutputBuilder<'a, I, OA> {
    pub(crate) fn new<F: Fn(&I) -> bool + 'a>(filter: F, len: usize) -> Self {
        let p = StableUniquePrioContainerMax::new(len);
        let filter = Box::new(filter);
        let output_add = OA::default();
        Self {
            p,
            filter,
            output_add,
        }
    }

    #[inline]
    pub fn push(&mut self, item: ResultItem<I>) {
        if !(self.filter)(&item.item) {
            self.p.insert(item);
        }
    }
}

impl<'a, I: Eq + Hash + Clone, OA: Default> Pushable for OutputBuilder<'a, I, OA> {
    type Item = ResultItem<I>;

    #[inline]
    fn push(&mut self, i: Self::Item) {
        self.push(i)
    }
}
