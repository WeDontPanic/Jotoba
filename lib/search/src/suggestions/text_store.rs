use std::cmp::Ordering;

use super::store_item::Item;

/// Store text data
pub trait TextStore {
    type Item: Item;

    /// Returns the Item at [`pos`]
    fn get_at(&self, pos: usize) -> &Self::Item;

    /// Returns the amount of items stored in [`TextStore`]
    fn len(&self) -> usize;

    // Totally not stolen from stdlib
    fn binary_search_by<'a, F>(&'a self, mut f: F) -> usize
    where
        F: FnMut(&'a dyn Item) -> std::cmp::Ordering,
    {
        let mut size = self.len();
        let mut left = 0;
        let mut right = size;

        while left < right {
            let mid = left + size / 2;

            let cmp = f(self.get_at(mid));

            if cmp == Ordering::Less {
                left = mid + 1;
            } else if cmp == Ordering::Greater {
                right = mid;
            } else {
                return mid;
            }

            size = right - left;
        }
        left
    }
}

impl<T: Item> TextStore for &Vec<T> {
    type Item = T;

    fn len(&self) -> usize {
        (*self).len()
    }

    fn get_at(&self, pos: usize) -> &T {
        // Its safe. Trust me
        unsafe { self.get_unchecked(pos) }
    }
}

impl<T: Item> TextStore for Vec<T> {
    type Item = T;

    fn len(&self) -> usize {
        self.len()
    }

    fn get_at(&self, pos: usize) -> &T {
        // Its safe. Trust me
        unsafe { self.get_unchecked(pos) }
    }
}

impl<T: Item> TextStore for &[T] {
    type Item = T;

    fn len(&self) -> usize {
        (*self).len()
    }

    fn get_at(&self, pos: usize) -> &T {
        // Its safe. Trust me
        unsafe { self.get_unchecked(pos) }
    }
}
