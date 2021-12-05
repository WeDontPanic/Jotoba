use std::convert::TryFrom;

use serde::{Deserialize, Serialize};
use strum_macros::{AsRefStr, EnumString};

#[derive(Debug, PartialEq, Clone, Copy, AsRefStr, EnumString, Serialize, Deserialize, Hash)]
#[repr(u8)]
pub enum GType {
    #[strum(serialize = "lit")]
    Literal,
    #[strum(serialize = "fig")]
    Figurative,
    #[strum(serialize = "expl")]
    Explanation,
}

impl TryFrom<i32> for GType {
    type Error = ();
    fn try_from(i: i32) -> Result<Self, Self::Error> {
        Ok(match i {
            0 => Self::Literal,
            1 => Self::Figurative,
            2 => Self::Explanation,
            _ => return Err(()),
        })
    }
}

impl Into<i32> for GType {
    fn into(self) -> i32 {
        match self {
            Self::Literal => 0,
            Self::Figurative => 1,
            Self::Explanation => 2,
        }
    }
}
