use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum Border {
    Left,
    Right,
    Top,
    Bottom,
}

impl Border {
    #[inline]
    pub fn get_class(&self) -> &'static str {
        match self {
            Border::Left => "l",
            Border::Right => "r",
            Border::Top => "t",
            Border::Bottom => "b",
        }
    }
}
