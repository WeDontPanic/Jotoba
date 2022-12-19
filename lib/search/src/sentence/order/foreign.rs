use engine::relevance::{data::SortData, RelevanceEngine};
use sparse_vec::SpVec32;
use types::jotoba::{language::Language, sentences::Sentence};
use vsm::doc_vec::DocVector;

pub struct ForeignOrder {
    lang: Language,
}

impl ForeignOrder {
    pub fn new(lang: Language) -> Self {
        Self { lang }
    }
}

impl RelevanceEngine for ForeignOrder {
    type OutItem = &'static Sentence;
    type IndexItem = DocVector<u32>;
    type Query = SpVec32;

    fn score<'item, 'query>(
        &self,
        item: &SortData<'item, 'query, Self::OutItem, Self::IndexItem, Self::Query>,
    ) -> f32 {
        let mut rel = item.vec_similarity();

        if !item.item().has_translation(self.lang) {
            rel *= 0.8;
        }

        rel
    }
}
