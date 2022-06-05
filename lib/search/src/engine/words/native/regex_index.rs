use intmap::int_set::IntSet;
use log::info;
use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};
use std::{
    collections::{hash_map::Iter, HashMap, HashSet},
    fs::File,
    io::BufReader,
    path::Path,
};

// In-memory storage for japanese regex index
pub(super) static INDEX: OnceCell<RegexSearchIndex> = OnceCell::new();

pub fn load<P: AsRef<Path>>(path: P) {
    let file = File::open(path.as_ref().join("regex_index")).expect("Missing regex index");

    let index: RegexSearchIndex =
        bincode::deserialize_from(BufReader::new(file)).expect("Invaild regex index");

    info!("Loaded japanese regex index");
    INDEX.set(index).ok();
}

/// Special index to allow fast and efficient regex search queries.
#[derive(Serialize, Deserialize)]
pub struct RegexSearchIndex {
    data: HashMap<char, HashSet<u32>>,
}

impl RegexSearchIndex {
    /// Creates a new empty Index
    #[inline]
    pub fn new() -> Self {
        RegexSearchIndex {
            data: HashMap::new(),
        }
    }

    /// Returns an iterator over all items in the index
    #[inline]
    pub fn iter(&self) -> Iter<char, HashSet<u32>> {
        self.data.iter()
    }

    /// Get all indexed words using characters in `chars`
    pub fn find<'a>(&'a self, chars: &[char]) -> IntSet {
        if chars.is_empty() {
            return IntSet::new();
        }

        let mut out = IntSet::new();

        // Add words of first character to `out`
        let mut chars_iter = chars.iter();

        // We want to fill `out` with some values.
        loop {
            let first = match chars_iter.next() {
                Some(f) => f,
                None => break,
            };

            if let Some(v) = self.data.get(first) {
                out.reserve(v.len());
                out.extend(v.iter().copied());
                // exit first found character
                break;
            }
        }

        for v in chars_iter.filter_map(|c| self.data.get(c)) {
            out.retain(|i| v.contains(&i));
            if out.is_empty() {
                return IntSet::new();
            }
        }

        out
    }

    /// Adds a new term to the index
    #[inline]
    pub fn add_term(&mut self, term: &str, seq_id: u32) {
        for c in term.chars() {
            self.data.entry(c).or_default().insert(seq_id);
        }
    }
}

/// Returns the loaded japanese regex index
#[inline]
pub fn get() -> &'static RegexSearchIndex {
    unsafe { INDEX.get_unchecked() }
}
