use std::marker::PhantomData;

use super::pushable::Pushable;

pub trait CPushable {
    type Item;

    fn push(&mut self, i: Self::Item) -> bool;
}

impl<T: Pushable> CPushable for T {
    type Item = <Self as Pushable>::Item;

    #[inline]
    fn push(&mut self, i: Self::Item) -> bool {
        <Self as Pushable>::push(self, i);
        true
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
}

impl<T> CPushable for MaxCounter<T> {
    type Item = T;

    #[inline]
    fn push(&mut self, _i: Self::Item) -> bool {
        if self.val >= self.max {
            return false;
        }

        self.val += 1;
        true
    }
}
