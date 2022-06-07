use std::{error::Error, fs::File, io::BufReader, path::Path};

use config::Config;
use indexes::radical::RadicalIndex;
use log::info;
use once_cell::sync::OnceCell;

pub(super) static RADICAL_INDEX: OnceCell<RadicalIndex> = OnceCell::new();

/// Load the radical index
pub(crate) fn load(config: &Config) -> Result<(), Box<dyn Error>> {
    let file = File::open(Path::new(config.get_indexes_source()).join("radical_index"))?;
    let index: RadicalIndex = bincode::deserialize_from(BufReader::new(file))?;
    RADICAL_INDEX.set(index).ok();
    info!("Loaded radical index");
    Ok(())
}

/// Returns the radical index
#[inline(always)]
pub fn get_index() -> &'static RadicalIndex {
    // Safety: This value never gets written and only set once at startup
    unsafe { RADICAL_INDEX.get_unchecked() }
}
