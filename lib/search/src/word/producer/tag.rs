use types::jotoba::{search::guess::Guess, words::Word};

use crate::{
    engine::{
        result_item::ResultItem,
        search_task::pushable::{Counter, Pushable},
    },
    executor::{out_builder::OutputBuilder, producer::Producer, searchable::Searchable},
    query::{Query, Tag},
    word::Search,
};

pub struct TagProducer<'a> {
    query: &'a Query,
}

impl<'a> TagProducer<'a> {
    pub fn new(query: &'a Query) -> Self {
        Self { query }
    }

    fn find_to<P>(&self, out: &mut P)
    where
        P: Pushable<Item = ResultItem<&'static Word>>,
    {
        self.push_jlpt(out);
        self.push_irr_ichidan(out);
    }

    fn push_jlpt<P>(&self, out: &mut P)
    where
        P: Pushable<Item = ResultItem<&'static Word>>,
    {
        let jlpt = match self.query.tags.iter().filter_map(|i| i.as_jlpt()).nth(0) {
            Some(j) => j.min(5).max(1),
            None => return,
        };

        for w in resources::get().words().by_jlpt(jlpt) {
            let len = w.get_reading().len();
            let rel = 1000usize.saturating_sub(len);
            out.push(ResultItem::new(w, rel))
        }
    }

    fn push_irr_ichidan<P>(&self, out: &mut P)
    where
        P: Pushable<Item = ResultItem<&'static Word>>,
    {
        if !self.query.tags.contains(&Tag::IrregularIruEru) {
            return;
        }

        for w in resources::get().words().irregular_ichidan() {
            out.push(ResultItem::new(w, 0));
        }
    }
}

impl<'a> Producer for TagProducer<'a> {
    type Target = Search<'a>;

    fn produce(
        &self,
        out: &mut OutputBuilder<
            <Self::Target as Searchable>::Item,
            <Self::Target as Searchable>::OutputAdd,
        >,
    ) {
        self.find_to(out);
    }

    fn should_run(&self, _already_found: usize) -> bool {
        self.query.tags.iter().any(|i| i.is_producer())
    }

    fn estimate(&self) -> Option<types::jotoba::search::guess::Guess> {
        let mut counter = Counter::new();
        self.find_to(&mut counter);
        Some(Guess::with_limit(counter.val() as u32, 100))
    }
}
