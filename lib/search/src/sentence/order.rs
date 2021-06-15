use std::cmp::Ordering;

use super::result::Sentence;

/// Order sentences by a japanese input query
pub(super) struct NativeOrder<'a> {
    query: &'a str,
}

impl<'a> NativeOrder<'a> {
    pub(super) fn new(query: &'a str) -> Self {
        Self { query }
    }

    /// Do the sort magic
    pub(super) fn sort(&self, items: &mut Vec<Sentence>) {
        items.sort_by(|a, b| self.order(a, b));
    }

    fn order(&self, a: &Sentence, b: &Sentence) -> Ordering {
        let a_cont = self.contains_query(a);
        let b_cont = self.contains_query(b);

        if a_cont && !b_cont {
            Ordering::Less
        } else if !a_cont && b_cont {
            Ordering::Greater
        } else {
            a.content.len().cmp(&b.content.len())
        }
    }

    fn contains_query(&self, a: &Sentence) -> bool {
        a.content.contains(self.query)
    }
}
