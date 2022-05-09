use log::info;
use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};
use std::{
    collections::{BTreeSet, HashMap},
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
    data: HashMap<char, Vec<u32>>,
}

impl RegexSearchIndex {
    /// Creates a new empty Index
    #[inline]
    pub fn new() -> Self {
        RegexSearchIndex {
            data: HashMap::new(),
        }
    }

    /// Adds a new term to the index. The `id` is supposed to be used to resolve `term`
    #[inline]
    pub fn add_term(&mut self, term: &str, seq_id: u32) {
        for c in term.chars() {
            self.data.entry(c).or_default().push(seq_id);
        }
    }

    pub fn finish(&mut self) {
        for (_, v) in self.data.iter_mut() {
            v.sort_unstable();
            v.dedup();
        }
    }

    /// Get all indexed words using characters in `chars`
    pub fn find<'a>(&'a self, chars: &[char]) -> Vec<u32> {
        if chars.is_empty() {
            return vec![];
        }

        let mut out: BTreeSet<u32> = BTreeSet::new();

        // Add words of first character to `out`
        let mut chars_iter = chars.iter();

        // We want to fill `out` with some values.
        loop {
            let first = match chars_iter.next() {
                Some(f) => f,
                None => break,
            };

            if let Some(v) = self.data.get(first) {
                out.extend(v.iter());
                // exit first found character
                break;
            }
        }

        for c in chars_iter {
            if let Some(v) = self.data.get(c) {
                if out.is_empty() {
                    return vec![];
                }
                out.retain(|i| v.contains(i));
            }
        }

        out.into_iter().collect()
    }
}

/// Returns the loaded japanese regex index
#[inline]
pub fn get() -> &'static RegexSearchIndex {
    unsafe { INDEX.get_unchecked() }
}
