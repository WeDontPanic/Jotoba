use std::cmp::Ordering;

pub struct ResultIter<'a, C, B, T>
where
    C: FnMut(&T) -> Ordering + Copy,
    B: BinarySearchable<Item = T>,
{
    cmp_fn: C,
    first: Option<usize>,
    item_pos: usize,
    find: &'a B,
}

impl<'a, C, B, T> Iterator for ResultIter<'a, C, B, T>
where
    C: FnMut(&T) -> Ordering + Copy,
    B: BinarySearchable<Item = T>,
{
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let curr_item_pos = self.first? + self.item_pos;

        if curr_item_pos >= self.find.len() {
            return None;
        }

        let item = self.find.get(curr_item_pos);
        if (self.cmp_fn)(&item) == Ordering::Equal {
            self.item_pos += 1;
            return Some(item);
        }

        None
    }
}

impl<'a, C, B, T> ResultIter<'a, C, B, T>
where
    C: FnMut(&T) -> Ordering + Copy,
    B: BinarySearchable<Item = T>,
{
    #[inline]
    pub(crate) fn new(cmp: C, search: &'a B, first: Option<usize>) -> Self {
        Self {
            cmp_fn: cmp,
            first,
            item_pos: 0,
            find: search,
        }
    }
}

/// A trait providing binary search for all `get` and `len` implementing types. Additionally
/// `search` can be used to retrieve all matching items in sorted order.
pub trait BinarySearchable: Sized {
    type Item: Sized;

    fn get(&self, pos: usize) -> Self::Item;
    fn len(&self) -> usize;

    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns an iterator over each matching result
    fn search<C>(&self, cmp: C) -> ResultIter<'_, C, Self, Self::Item>
    where
        C: FnMut(&Self::Item) -> Ordering + Copy,
    {
        let first_item = self.find_first(cmp);
        ResultIter::new(cmp, self, first_item)
    }

    fn binary_search_by<'a, F>(&'a self, mut f: F) -> Option<usize>
    where
        F: FnMut(&Self::Item) -> Ordering,
    {
        let mut size = self.len();
        let mut left = 0;
        let mut right = size;

        while left < right {
            let mid = left + size / 2;

            let cmp = f(&self.get(mid));

            if cmp == Ordering::Less {
                left = mid + 1;
            } else if cmp == Ordering::Greater {
                right = mid;
            } else {
                return Some(mid);
            }

            size = right - left;
        }
        None
    }

    /// Finds first matching item
    fn find_first<C>(&self, mut cmp: C) -> Option<usize>
    where
        C: FnMut(&Self::Item) -> Ordering,
    {
        // Find using binary search. If multiple results found (which is very likely the case in
        // our implementation), a random item of the matching ones will be found
        let random_index = self.binary_search_by(|a| cmp(a))?;

        let mut curr_pos = random_index.saturating_sub(100);

        loop {
            if cmp(&self.get(curr_pos)) != Ordering::Equal {
                loop {
                    curr_pos += 1;
                    if cmp(&self.get(curr_pos)) == Ordering::Equal {
                        break;
                    }
                }
                break Some(curr_pos);
            }

            if curr_pos == 0 {
                break None;
            }
            curr_pos = curr_pos.saturating_sub(200);
        }
    }
}
