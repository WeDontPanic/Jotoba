use super::Pushable;
use std::marker::PhantomData;

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
