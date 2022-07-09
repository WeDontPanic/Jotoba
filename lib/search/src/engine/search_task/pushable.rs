use std::{hash::Hash, marker::PhantomData};

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

pub struct Counter<T> {
    c: usize,
    p: PhantomData<T>,
}

impl<T> Counter<T> {
    #[inline]
    pub fn new() -> Self {
        Self {
            c: 0,
            p: PhantomData,
        }
    }

    #[inline]
    pub fn val(&self) -> usize {
        self.c
    }
}

impl<T> Pushable for Counter<T> {
    type Item = T;

    #[inline]
    fn push(&mut self, _i: Self::Item) {
        self.c += 1;
    }
}
