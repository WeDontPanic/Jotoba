pub mod retrieve;
pub mod storage;

pub use storage::ResourceStorage;

use once_cell::sync::OnceCell;
use std::{
    error::Error,
    fs::File,
    io::{BufReader, Write},
    path::Path,
};

/// InMemory storage for all data
static STORAGE: OnceCell<ResourceStorage> = OnceCell::new();

/// Get loaded storage data
#[inline(always)]
pub fn get() -> &'static ResourceStorage {
    // Safety:
    // The STORAGE cell gets initialized once at the beginning which is absolutely necessary for
    // the program to work. It won't be unset so its always safe
    unsafe { STORAGE.get_unchecked() }
}

/// Returns `true` if the storage is loaded
#[inline(always)]
pub fn is_loaded() -> bool {
    STORAGE.get().is_some()
}

/// Load the resource storage from a file. Returns `true` if it wasn't loaded before
pub fn load<P: AsRef<Path>>(path: P) -> Result<bool, Box<dyn Error>> {
    let mut reader = BufReader::new(File::open(path)?);
    let storage: ResourceStorage = bincode::deserialize_from(&mut reader)?;
    Ok(STORAGE.set(storage).is_ok())
}

/// Serializes a ResourceStorage into `output`
pub fn store<W: Write>(output: W, storage: &ResourceStorage) -> Result<(), Box<dyn Error>> {
    bincode::serialize_into(output, storage)?;
    Ok(())
}

/*
pub fn set(res_storage: ResourceStorage) {
    STORAGE.set(res_storage).ok();
}
*/
