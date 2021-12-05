/// Contains raw data structures used for parsing and generating the 'real' resources
#[cfg(feature = "raw_types")]
pub mod raw;

/// Contains all information holding structures for jotoba resources
pub mod jotoba;

/// Contains all structures and informations required for the API
#[cfg(feature = "api")]
pub mod api;
