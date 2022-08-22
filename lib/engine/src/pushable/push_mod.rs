use std::marker::PhantomData;

use super::Pushable;

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
