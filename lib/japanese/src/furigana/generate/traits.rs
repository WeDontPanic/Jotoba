pub trait ReadingRetrieve {
    fn onyomi(&self, lit: char) -> Vec<String>;
    fn kunyomi(&self, lit: char) -> Vec<String>;

    fn all(&self, lit: char) -> Vec<String> {
        self.kunyomi(lit)
            .into_iter()
            .chain(self.onyomi(lit).into_iter())
            .collect()
    }
}

impl<T: ReadingRetrieve> ReadingRetrieve for &T {
    fn onyomi(&self, lit: char) -> Vec<String> {
        (*self).onyomi(lit)
    }

    fn kunyomi(&self, lit: char) -> Vec<String> {
        (*self).kunyomi(lit)
    }
}
