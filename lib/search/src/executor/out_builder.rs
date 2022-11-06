use engine::{pushable::Pushable, relevance::item::RelItem};
use priority_container::StableUniquePrioContainerMax;
use std::hash::Hash;

pub struct OutputBuilder<'a, I, OA> {
    pub(crate) p: StableUniquePrioContainerMax<RelItem<I>>,
    pub(crate) filter: Box<dyn Fn(&I) -> bool + 'a>,
    pub(crate) output_add: OA,
    pub(crate) rel_list: Vec<f32>,
    pub(crate) max: f32,
}

impl<'a, I: Eq + Hash + Clone, OA: OutputAddable> OutputBuilder<'a, I, OA> {
    #[inline]
    pub(crate) fn new<F: Fn(&I) -> bool + 'a>(filter: F, len: usize) -> Self {
        Self {
            p: StableUniquePrioContainerMax::new(len),
            filter: Box::new(filter),
            output_add: OA::default(),
            rel_list: vec![],
            max: 0.0,
        }
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.p.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.p.is_empty()
    }

    /// Pushes an element into the output and  returns `true` if it was not filtered out
    #[inline]
    pub fn push(&mut self, item: RelItem<I>) -> bool {
        if !(self.filter)(&item.item) {
            if self.max < item.relevance {
                self.max = item.relevance;
            }

            let rel = item.relevance;
            if self.p.insert(item) {
                self.rel_list.push(rel);
            }

            return true;
        }

        false
    }
}

impl<'a, I: Eq + Hash + Clone, OA: OutputAddable> Pushable for OutputBuilder<'a, I, OA> {
    type Item = RelItem<I>;

    /// Pushes an element into the output and  returns `true` if it was not filtered out
    #[inline]
    fn push(&mut self, i: Self::Item) -> bool {
        self.push(i)
    }
}

pub trait OutputAddable: Default {
    #[inline]
    fn is_empty(&self) -> bool {
        false
    }
}

impl OutputAddable for () {}
