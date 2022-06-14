use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Request {
    pub literal: char,
    pub full: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OutObject {
    name: char,
    clickable: bool,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    children: Vec<OutObject>,
}

impl OutObject {
    #[inline]
    pub fn new(name: char) -> Self {
        Self {
            name,
            children: vec![],
            clickable: false,
        }
    }

    #[inline]
    pub fn with_children(name: char, children: Vec<OutObject>) -> Self {
        Self {
            name,
            children,
            clickable: false,
        }
    }

    #[inline]
    pub fn add_child(&mut self, child: Self) {
        self.children.push(child)
    }

    pub fn set_clickable(&mut self, clickable: bool) {
        self.clickable = clickable;
    }
}
