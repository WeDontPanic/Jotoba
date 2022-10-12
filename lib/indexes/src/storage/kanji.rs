use crate::kanji::reading_freq::FrequencyIndex;
use std::{error::Error, fs::File, io::BufReader, path::Path};

pub const K_READINGS_FREQ_FILE: &str = "kreading_freq_index";

/// Store for name indexes
pub struct KanjiStore {
    kread_frequency: FrequencyIndex,
}

impl KanjiStore {
    pub fn new(kread_frequency: FrequencyIndex) -> Self {
        Self { kread_frequency }
    }

    #[inline(always)]
    pub fn reading_freq(&self) -> &FrequencyIndex {
        &self.kread_frequency
    }
}

pub(crate) fn load<P: AsRef<Path>>(path: P) -> Result<KanjiStore, Box<dyn Error + Send + Sync>> {
    let kread_file = Path::new(path.as_ref()).join(K_READINGS_FREQ_FILE);
    let kread_frequency: FrequencyIndex =
        bincode::deserialize_from(BufReader::new(File::open(kread_file)?))?;
    Ok(KanjiStore::new(kread_frequency))
}
