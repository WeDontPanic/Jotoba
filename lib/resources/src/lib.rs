pub mod models;
pub mod parse;

use models::storage::ResourceStorage;
use once_cell::sync::OnceCell;
use std::{error::Error, fs::File, io::BufReader, path::Path};

/// Public storage of all resourcen in memory/swap
static RESOURCES: OnceCell<ResourceStorage> = OnceCell::new();

/// Initializes the memory storage
pub fn initialize_resources<P: AsRef<Path>>(path: P) -> Result<(), Box<dyn Error>> {
    RESOURCES
        .set(models::load_stoarge(BufReader::new(File::open(path)?))?)
        .ok()
        .expect("Storage already initialized");
    Ok(())
}

/// Returns the `ResourceStorage`
#[inline]
pub fn get() -> &'static ResourceStorage {
    // Safety:
    // The RESOURCE cell gets initialized once at the beginning which is absolutely necessary for
    // the program to work at all. It can't get changed later on since its private.
    unsafe { RESOURCES.get_unchecked() }
}
