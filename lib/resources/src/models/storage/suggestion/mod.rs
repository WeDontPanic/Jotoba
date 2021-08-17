use std::{
    fs::File,
    io::{BufReader, Read},
    marker::PhantomData,
    path::Path,
};

use indexed_file::{any::IndexedReader, Indexable, ReadByLine};
use utils::binary_search::BinarySearchable;

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
}

impl<T: From<Vec<u8>>> BinarySearchable for SuggestionDictionary<T> {
    type Item = T;

    #[inline]
    fn get(&self, pos: usize) -> Self::Item {
        self.get(pos).unwrap()
    }

    #[inline]
    fn len(&self) -> usize {
        self.len()
    }
}
