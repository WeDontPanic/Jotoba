use serde::Serialize;

use crate::jotoba::names::Name;

/// Names API response. Contains all Names
#[derive(Clone, Debug, Serialize)]
pub struct Response {
    names: Vec<Name>,
}

impl Response {
    #[inline]
    pub fn new(names: Vec<Name>) -> Self {
        Self { names }
    }
}
