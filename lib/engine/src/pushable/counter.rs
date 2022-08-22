use std::marker::PhantomData;

use super::Pushable;

/// Counts all push calls without storing the items
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
    fn push(&mut self, _: Self::Item) -> bool {
        self.c += 1;
        true
    }
}
