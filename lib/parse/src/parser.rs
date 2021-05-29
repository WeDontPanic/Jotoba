use std::io::BufRead;

use crate::error::{self, Error};

/// Trait to build the basic contsruct
/// of a parser.
pub trait Parse<R, T>
where
    Self: Sized,
    R: BufRead,
{
    /// Create a new parser instance
    fn new(reader: R) -> Self;

    /// Parse a given source
    fn parse<F>(self, f: F) -> Result<Self, Error>
    where
        F: Fn(T, usize) -> bool;

    /// Return the amount of items, the parser would return
    /// on a full parse
    fn count(self) -> Result<usize, error::Error> {
        Ok(0)
    }
}
