use super::relevance::item::RelItem;
use priority_container::StableUniquePrioContainerMax;
use std::{hash::Hash, marker::PhantomData};

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
    fn push(&mut self, i: Self::Item) -> bool {
        self.output.push((self.f)(i))
    }
}

/// A counter that Implements CancelPushable which counts up to a fixed value and
/// Cancels counting if this value has been reached
pub struct MaxCounter<T> {
    val: usize,
    max: usize,
    p: PhantomData<T>,
}

impl<T> MaxCounter<T> {
    #[inline]
    pub fn new(max: usize) -> Self {
        Self {
            val: 0,
            max,
            p: PhantomData,
        }
    }

    #[inline]
    pub fn val(&self) -> usize {
        self.val
    }

    #[inline]
    pub fn inc(&mut self, delta: usize) {
        self.val += delta;
    }

    #[inline]
    pub fn is_full(&self) -> bool {
        self.val >= self.max
    }
}

impl<T> Pushable for MaxCounter<T> {
    type Item = T;

    #[inline]
    fn push(&mut self, _i: Self::Item) -> bool {
        if self.is_full() {
            return false;
        }

        self.val += 1;
        true
    }
}

/// A counter that Implements CancelPushable which counts up to a fixed value and
/// Cancels counting if this value has been reached
pub struct FilteredMaxCounter<'a, T> {
    val: usize,
    max: usize,
    pub filter: Box<dyn Fn(&T) -> bool + 'a>,
    p: PhantomData<T>,
}

impl<'a, T> FilteredMaxCounter<'a, T> {
    #[inline]
    pub fn new<F>(max: usize, filter: F) -> Self
    where
        F: Fn(&T) -> bool + 'a,
    {
        Self {
            val: 0,
            max,
            filter: Box::new(filter),
            p: PhantomData,
        }
    }

    #[inline]
    pub fn val(&self) -> usize {
        self.val
    }

    #[inline]
    pub fn inc(&mut self, delta: usize) {
        self.val += delta;
    }

    #[inline]
    pub fn is_full(&self) -> bool {
        self.val >= self.max
    }
}

impl<'a, T> Pushable for FilteredMaxCounter<'a, T> {
    type Item = T;

    #[inline]
    fn push(&mut self, i: Self::Item) -> bool {
        if self.is_full() {
            return false;
        }

        if !(self.filter)(&i) {
            self.val += 1;
        }

        true
    }
}
