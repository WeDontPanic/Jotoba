use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Request {
    pub literal: char,
    pub full: bool,
}

#[derive(Deserialize, Serialize)]
pub struct Response {
    tree: OutObject,
    has_big: bool,
}

impl Response {
    pub fn new(tree: OutObject, has_big: bool) -> Self {
        Self { tree, has_big }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct OutObject {
    name: char,
    literal_available: bool,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    children: Vec<OutObject>,
}

impl OutObject {
    #[inline]
    pub fn new(name: char) -> Self {
        Self {
            name,
            children: vec![],
            literal_available: false,
        }
    }

    #[inline]
    pub fn with_children(name: char, children: Vec<OutObject>) -> Self {
        Self {
            name,
            children,
            literal_available: false,
        }
    }

    #[inline]
    pub fn add_child(&mut self, child: Self) {
        self.children.push(child)
    }

    #[inline]
    pub fn set_literal_available(&mut self, literal_available: bool) {
        self.literal_available = literal_available;
    }
}
