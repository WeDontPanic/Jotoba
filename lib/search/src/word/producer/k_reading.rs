use engine::{
    pushable::FilteredMaxCounter,
    pushable::{PushMod, Pushable},
    relevance::item::RelItem,
    task::SearchTask,
};
use types::jotoba::{kanji::Kanji, words::Word};

use crate::{
    engine::words::native::k_reading,
    executor::{out_builder::OutputBuilder, producer::Producer, searchable::Searchable},
    query::Query,
    word::{order::kanji_reading::KanjiReadingRelevance, Search},
};

/// Kanji reading search producer
pub struct KReadingProducer<'a> {
    query: &'a Query,
}

impl<'a> KReadingProducer<'a> {
    pub fn new(query: &'a Query) -> Self {
        Self { query }
    }

    /// Returns the kanji from the search. Returns `None` if kanji does not exist or doesn't
    /// match the reading from the search
    fn get_kanji(&self) -> Option<&'static Kanji> {
        let reading = self.query.form.as_kanji_reading()?;
        let kanji_storage = resources::get().kanji();

        let kanji = kanji_storage.by_literal(reading.literal)?;
        kanji.has_reading(&reading.reading).then(|| kanji)
    }

    /// Returns a query for the kanji reading index for the search query
    fn kr_query(&self) -> Option<String> {
        let kanji = self.get_kanji()?;
        let reading = self.query.form.as_kanji_reading().unwrap();
        Some(format!("{}{}", kanji.literal, reading.reading))
    }

    fn find_to<P>(&self, out: &mut P)
    where
        P: Pushable<Item = RelItem<&'static Word>>,
    {
        let engine_query = match self.kr_query() {
            Some(q) => q,
            None => return,
        };

        SearchTask::<k_reading::Engine>::new(&engine_query)
            .with_custom_order(KanjiReadingRelevance)
            .find_to(out);
    }
}

impl<'a> Producer for KReadingProducer<'a> {
    type Target = Search<'a>;

    fn produce(
        &self,
        out: &mut OutputBuilder<
            <Self::Target as Searchable>::Item,
            <Self::Target as Searchable>::ResAdd,
        >,
    ) {
        self.find_to(out);
    }

    fn should_run(&self, _already_found: usize) -> bool {
        self.query.form.is_kanji_reading()
    }

    fn estimate_to(&self, out: &mut FilteredMaxCounter<<Self::Target as Searchable>::Item>) {
        let mut m = PushMod::new(out, |i: RelItem<&Word>| i.item);
        // TODO: use estimate_to here
        self.find_to(&mut m);
    }
}
