use engine::relevance::{data::SortData, RelevanceEngine};
use sparse_vec::SpVec32;
use types::jotoba::{languages::Language, sentences::Sentence};
use vsm::doc_vec::DocVector;

pub struct NativeOrder {
    lang: Language,
}

impl NativeOrder {
    pub fn new(lang: Language) -> Self {
        Self { lang }
    }
}

impl RelevanceEngine for NativeOrder {
    type OutItem = &'static Sentence;
    type IndexItem = DocVector<u32>;
    type Query = SpVec32;

    fn score<'item, 'query>(
        &self,
        item: &SortData<'item, 'query, Self::OutItem, Self::IndexItem, Self::Query>,
    ) -> f32 {
        let mut rel = item.vec_similarity();

        if !item.item().has_translation(self.lang) {
            rel *= 0.99;
        }

        rel * 1_000_000.0
    }
}
