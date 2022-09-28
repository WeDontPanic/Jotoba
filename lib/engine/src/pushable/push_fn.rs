use super::Pushable;
use std::marker::PhantomData;

pub struct PushFn<F, T> {
    f: F,
    p: PhantomData<T>,
}

impl<F, T> PushFn<F, T>
where
    F: FnMut(T) -> bool,
{
    #[inline]
    pub fn new(f: F) -> Self {
        Self { f, p: PhantomData }
    }
}

impl<F, T> Pushable for PushFn<F, T>
where
    F: FnMut(T) -> bool,
{
    type Item = T;

    #[inline]
    fn push(&mut self, i: Self::Item) -> bool {
        (self.f)(i)
    }
}
