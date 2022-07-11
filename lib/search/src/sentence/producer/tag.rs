use crate::{
    engine::{
        result_item::ResultItem,
        search_task::{
            cpushable::FilteredMaxCounter,
            pushable::{PushMod, Pushable},
        },
    },
    executor::{out_builder::OutputBuilder, producer::Producer, searchable::Searchable},
    query::Query,
    sentence::Search,
};
use types::jotoba::{languages::Language, sentences::Sentence};

/// Producer for Tags
pub struct TagProducer<'a> {
    query: &'a Query,
}

impl<'a> TagProducer<'a> {
    pub fn new(query: &'a Query) -> Self {
        Self { query }
    }

    fn jlpt(&self) -> Option<u8> {
        self.query
            .tags
            .iter()
            .find(|i| i.is_jlpt())
            .map(|i| i.as_jlpt().unwrap())
    }

    fn jlpt_iter(&self, jlpt: u8) -> impl Iterator<Item = &'static Sentence> + 'a {
        resources::get()
            .sentences()
            .ids_by_jlpt(jlpt)
            .filter_map(|i| resources::get().sentences().by_id(i))
            .filter(|sentence| {
                sentence.has_translation(self.query.settings.user_lang)
                    && (sentence.has_translation(Language::English)
                        && self.query.settings.show_english)
            })
            .take(10000)
    }

    fn find_to<P>(&self, out: &mut P)
    where
        P: Pushable<Item = ResultItem<&'static Sentence>>,
    {
        if let Some(jlpt) = self.jlpt() {
            for sentence in self.jlpt_iter(jlpt) {
                out.push(ResultItem::new(sentence, 0));
            }
        }
    }
}

impl<'a> Producer for TagProducer<'a> {
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

    fn estimate_to(&self, out: &mut FilteredMaxCounter<<Self::Target as Searchable>::Item>) {
        let mut m = PushMod::new(out, |i: ResultItem<&Sentence>| i.item);
        self.find_to(&mut m);
    }

    fn should_run(&self, _already_found: usize) -> bool {
        // Only run for jlpt tags
        self.query.tags.iter().any(|i| i.is_jlpt())
    }
}
