use crate::{
    engine::{
        names::{foreign, native},
        SearchTask,
    },
    executor::{out_builder::OutputBuilder, producer::Producer, searchable::Searchable},
    name::Search,
    query::{Query, QueryLang},
};

pub struct NameProducer<'a> {
    query: &'a Query,
}

impl<'a> NameProducer<'a> {
    pub fn new(query: &'a Query) -> Self {
        Self { query }
    }

    fn jp_task(&self) -> SearchTask<native::Engine> {
        SearchTask::<native::Engine>::new(&self.query.query_str).threshold(0.05f32)
    }

    fn foreign_task(&self) -> SearchTask<foreign::Engine> {
        SearchTask::<foreign::Engine>::new(&self.query.query_str).threshold(0.05f32)
    }
}

impl<'a> Producer for NameProducer<'a> {
    type Target = Search<'a>;

    fn produce(
        &self,
        out: &mut OutputBuilder<
            <Self::Target as Searchable>::Item,
            <Self::Target as Searchable>::ResAdd,
        >,
    ) {
        if self.query.q_lang == QueryLang::Japanese {
            self.jp_task().find_to(out);
        } else {
            self.foreign_task().find_to(out);
        }
    }

    fn estimate(&self) -> Option<types::jotoba::search::guess::Guess> {
        if self.query.q_lang == QueryLang::Japanese {
            self.jp_task().estimate_result_count().ok()
        } else {
            self.foreign_task().estimate_result_count().ok()
        }
    }
}
