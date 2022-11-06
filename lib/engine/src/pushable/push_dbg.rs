use super::Pushable;
use std::{fmt::Debug, marker::PhantomData};

/// Allows debugging pushed items
pub struct PushDbg<'a, P, I> {
    output: &'a mut P,
    p: PhantomData<I>,
}

impl<'a, P, I> PushDbg<'a, P, I>
where
    P: Pushable<Item = I>,
    I: Debug,
{
    pub fn new(output: &'a mut P) -> Self {
        Self {
            output,
            p: PhantomData,
        }
    }
}

impl<'a, P, I> Pushable for PushDbg<'a, P, I>
where
    P: Pushable<Item = I>,
    I: Debug,
{
    type Item = I;

    #[inline]
    fn push(&mut self, i: Self::Item) -> bool {
        print!("{i:#?}");
        let cont = self.output.push(i);
        println!(" continue: {cont}");
        cont
    }
}
