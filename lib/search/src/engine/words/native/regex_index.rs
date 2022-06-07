use indexes::regex::RegexSearchIndex;
use log::info;
use once_cell::sync::OnceCell;
use std::{fs::File, io::BufReader, path::Path};

// In-memory storage for japanese regex index
pub(super) static INDEX: OnceCell<RegexSearchIndex> = OnceCell::new();

pub fn load<P: AsRef<Path>>(path: P) {
    let file = File::open(path.as_ref().join("regex_index")).expect("Missing regex index");

    let index: RegexSearchIndex =
        bincode::deserialize_from(BufReader::new(file)).expect("Invaild regex index");

    info!("Loaded japanese regex index");
    INDEX.set(index).ok();
}

/// Returns the loaded japanese regex index
#[inline]
pub fn get() -> &'static RegexSearchIndex {
    unsafe { INDEX.get_unchecked() }
}
