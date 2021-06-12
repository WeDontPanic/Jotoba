/// An item which can be stored within [`TextStore`]
pub trait Item {
    fn get_text(&self) -> &str;
}

impl Item for &String {
    fn get_text(&self) -> &str {
        self
    }
}

impl Item for String {
    fn get_text(&self) -> &str {
        &self
    }
}

impl Item for &str {
    fn get_text(&self) -> &str {
        self
    }
}
