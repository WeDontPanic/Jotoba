use std::{
    cmp::Ordering,
    fs::File,
    io::{BufReader, Read},
    marker::PhantomData,
    path::Path,
};

use indexed_file::{any::IndexedReader, Indexable, ReadByLine};

pub mod provider;

#[derive(Clone)]
pub struct SuggestionDictionary<I: From<Vec<u8>>> {
    data: IndexedReader<Vec<u8>>,
    phantom: PhantomData<I>,
}

impl<I: From<Vec<u8>>> SuggestionDictionary<I> {
    /// Loads a SuggestionDictionary from a file
    pub(crate) fn load<P: AsRef<Path>>(file: P) -> Result<Self, indexed_file::error::Error> {
        let mut data = Vec::new();
        BufReader::new(File::open(file.as_ref())?).read_to_end(&mut data)?;

        Ok(Self {
            data: IndexedReader::new(data)?,
            phantom: PhantomData,
        })
    }

    /// Gets the amount of suggestion entries in the suggestion-dictionary
    #[inline]
    pub fn len(&self) -> usize {
        self.data.total_lines()
    }

    /// Returns a suggestion item at `pos` or `None` if out of bounds
    pub fn get(&self, pos: usize) -> Option<I> {
        let mut data = Vec::with_capacity(16);
        self.data.clone().read_line_raw(pos, &mut data).ok()?;
        Some(I::from(data))
    }

    /// Returns an iterator over each matching result
    pub fn search<'a, C>(&'a self, mut cmp: C) -> impl Iterator<Item = I> + 'a
    where
        C: FnMut(&I) -> Ordering + 'a + Copy,
    {
        let first_item = self.find_first(cmp);

        let mut item_pos = 0;
        std::iter::from_fn(move || {
            let first_item = first_item?;
            let curr_item_pos = first_item + item_pos;

            if curr_item_pos >= self.len() {
                return None;
            }

            let item = self.get(curr_item_pos)?;
            if cmp(&item) == Ordering::Equal {
                item_pos += 1;
                return Some(item);
            }

            None
        })
    }

    fn binary_search_by<'a, F>(&'a self, mut f: F) -> Option<usize>
    where
        F: FnMut(I) -> Ordering,
    {
        let mut size = self.len();
        let mut left = 0;
        let mut right = size;

        while left < right {
            let mid = left + size / 2;

            let cmp = f(self.get(mid).unwrap());

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
        C: FnMut(&I) -> Ordering,
    {
        // Find using binary search. If multiple results found (which is very likely the case in
        // our implementation), a random item of the matching ones will be found
        let random_index = self.binary_search_by(|a| cmp(&a))?;

        let mut curr_pos = random_index.saturating_sub(100);

        loop {
            if cmp(&self.get(curr_pos)?) != Ordering::Equal {
                loop {
                    curr_pos += 1;
                    if cmp(&self.get(curr_pos)?) == Ordering::Equal {
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
