use crate::{
    engine::result_item::ResultItem,
    executor::{out_builder::OutputBuilder, producer::Producer, searchable::Searchable},
    query::Query,
    sentence::Search,
};
use types::jotoba::{languages::Language, search::guess::Guess, sentences::Sentence};

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
        if let Some(jlpt) = self.jlpt() {
            for sentence in self.jlpt_iter(jlpt) {
                out.push(ResultItem::new(sentence, 0));
            }
        }
    }

    fn should_run(&self, _already_found: usize) -> bool {
        // Only run for jlpt tags
        self.query.tags.iter().any(|i| i.is_jlpt())
    }

    fn estimate(&self) -> Option<types::jotoba::search::guess::Guess> {
        let mut len = 0;
        if let Some(jlpt) = self.jlpt() {
            len = self.jlpt_iter(jlpt).count() as u32;
        }

        Some(Guess::with_limit(len, 100))
    }
}
