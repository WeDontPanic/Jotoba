use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Request {
    pub literal: char,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OutObject {
    name: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    children: Vec<OutObject>,
}

impl OutObject {
    pub fn new(name: String) -> Self {
        Self {
            name,
            children: vec![],
        }
    }

    #[inline]
    pub fn with_children(name: String, children: Vec<OutObject>) -> Self {
        Self { name, children }
    }

    #[inline]
    pub fn add_children<I: IntoIterator<Item = OutObject>>(&mut self, children: I) {
        self.children.extend(children);
    }

    #[inline]
    pub fn add_child(&mut self, child: Self) {
        self.children.push(child)
    }
}
