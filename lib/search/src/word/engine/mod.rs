pub mod document;
pub(crate) mod foreign;
pub mod result;

use config::Config;

/// Load all indexes for word search engine
pub fn load_indexes(config: &Config) -> Result<(), Box<dyn std::error::Error>> {
    foreign::index::load(config)?;
    Ok(())
}
