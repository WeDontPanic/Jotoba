use serde::Serialize;

use crate::jotoba::words::inflection::Inflection;

#[derive(Clone, Serialize)]
pub struct InflectionInfo {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    inflections: Vec<Inflection>,
    /// The "uninflected" version
    lexeme: String,
}

impl InflectionInfo {
    /// Create a new InflectionInfo
    #[inline]
    pub fn new(inflection: Vec<Inflection>, lexeme: String) -> Self {
        Self {
            inflections: inflection,
            lexeme,
        }
    }
}
