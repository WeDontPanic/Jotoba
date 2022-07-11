use crate::engine::result_item::ResultItem;
use priority_container::StableUniquePrioContainerMax;
use std::{hash::Hash, marker::PhantomData};

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

/// Allows modifying pushed data
pub struct PushMod<'a, P, I, O, F> {
    output: &'a mut P,
    f: F,
    p: PhantomData<I>,
    p2: PhantomData<O>,
}

impl<'a, P, I, O, F> PushMod<'a, P, I, O, F>
where
    P: Pushable<Item = O>,
    F: Fn(I) -> O,
{
    pub fn new(output: &'a mut P, f: F) -> Self {
        Self {
            output,
            f,
            p: PhantomData,
            p2: PhantomData,
        }
    }
}

impl<'a, P, I, O, F> Pushable for PushMod<'a, P, I, O, F>
where
    F: Fn(I) -> O,
    P: Pushable<Item = O>,
{
    type Item = I;

    #[inline]
    fn push(&mut self, i: Self::Item) {
        self.output.push((self.f)(i));
    }
}
