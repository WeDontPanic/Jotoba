use super::pushable::Pushable;
use std::marker::PhantomData;

pub trait CPushable {
    type Item;

    fn push(&mut self, i: Self::Item) -> bool;
}

impl<T: CPushable> Pushable for T {
    type Item = <Self as CPushable>::Item;

    #[inline]
    fn push(&mut self, i: Self::Item) {
        <Self as CPushable>::push(self, i);
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

impl<T> CPushable for MaxCounter<T> {
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

impl<'a, T> CPushable for FilteredMaxCounter<'a, T> {
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
