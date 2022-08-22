use itertools::Itertools;
use types::jotoba::words::Word;

use crate::{
    engine::words::native::regex,
    executor::{out_builder::OutputBuilder, producer::Producer, searchable::Searchable},
    query::{regex::RegexSQuery, Query},
    word::{order::regex_order, Search},
};
use engine::{
    pushable::FilteredMaxCounter,
    pushable::{PushMod, Pushable},
    relevance::item::RelItem,
};

pub struct RegexProducer<'a> {
    query: &'a Query,
}

impl<'a> RegexProducer<'a> {
    pub fn new(query: &'a Query) -> Self {
        Self { query }
    }

    fn find_to_unsorted<P: Pushable<Item = RelItem<&'static Word>>>(
        &self,
        out: &mut P,
    ) -> Option<()> {
        let regex_query = self.query.as_regex_query()?;
        search(&regex_query, |_, _| 0, out);
        Some(())
    }

    fn find_to<P: Pushable<Item = RelItem<&'static Word>>>(&self, out: &mut P) -> Option<()> {
        let regex_query = self.query.as_regex_query()?;
        search(&regex_query, |w, r| regex_order(w, r, &regex_query), out);
        Some(())
    }
}

impl<'a> Producer for RegexProducer<'a> {
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
        self.query.as_regex_query().is_some()
    }

    fn estimate_to(&self, out: &mut FilteredMaxCounter<<Self::Target as Searchable>::Item>) {
        let mut mid = PushMod::new(out, |i: RelItem<&'static Word>| i.item);
        self.find_to_unsorted(&mut mid);
    }
}

pub fn search<'a, F, P>(query: &'a RegexSQuery, sort: F, out: &mut P)
where
    F: Fn(&'a Word, &'a str) -> usize,
    P: Pushable<Item = RelItem<&'static Word>>,
{
    let word_resources = resources::get().words();

    let index = indexes::get().word().regex();
    let possible_results = regex::find_words(index, &query.get_chars());

    for seq_id in possible_results.into_iter().sorted() {
        let word = word_resources.by_sequence(seq_id).unwrap();

        let item_iter = word
            .reading_iter(true)
            .filter_map(|i| query.matches(&i.reading).then(|| (word, &i.reading)))
            .map(|(word, reading)| {
                let order = sort(word, reading) as f32;
                RelItem::new(word, order)
            });

        for i in item_iter {
            out.push(i);
        }
    }
}
