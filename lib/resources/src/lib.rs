pub mod models;
pub mod news;
pub mod parse;

use models::storage::ResourceStorage;
use once_cell::sync::OnceCell;
use std::{error::Error, path::Path};

/// Public storage of all resourcen in memory/swap
static RESOURCES: OnceCell<ResourceStorage> = OnceCell::new();

/// Initializes the memory storage
pub fn initialize_resources<P: AsRef<Path>>(
    dict_data_path: P,
    suggestions_path: P,
    rad_map_path: P,
    sentences_path: P,
) -> Result<(), Box<dyn Error>> {
    let storage = models::load_storage(
        dict_data_path,
        suggestions_path,
        rad_map_path,
        sentences_path,
    )?;

    RESOURCES
        .set(storage)
        .ok()
        .expect("Storage already initialized");

    Ok(())
}

/// Returns the `ResourceStorage`
#[inline]
pub fn get() -> &'static ResourceStorage {
    // Safety:
    // The RESOURCE cell gets initialized once at the beginning which is absolutely necessary for
    // the program to work. It can't and won't get changed later on since its private.
    unsafe { RESOURCES.get_unchecked() }
}

pub fn set(res_storage: ResourceStorage) {
    RESOURCES.set(res_storage).ok();
}
