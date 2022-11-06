use engine::relevance::{data::SortData, RelevanceEngine};
use sparse_vec::{SpVec32, VecExt};
use types::jotoba::{languages::Language, sentences::Sentence};
use vsm::doc_vec::DocVector;

pub const QUERY_WEIGHT: f32 = 100.0;

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

    #[inline]
    fn score<'item, 'query>(
        &self,
        item: &SortData<'item, 'query, Self::OutItem, Self::IndexItem, Self::Query>,
    ) -> f32 {
        //let mut rel = term_dist(item.query(), item.index_item().vec());
        let mut rel = sim(item.query(), item.index_item().vec(), QUERY_WEIGHT);

        if !item.item().has_translation(self.lang) {
            rel *= 0.99;
        }

        rel
    }
}

/// Calculates a similar value to the cosine similarity between vec_a and vec_b but
/// gives the length more weight than vec_b's length.
/// This prevents longer sentences being less relevant than short sentences, even if
/// the longer sentences contains all terms of the query when the short sentence does not.
#[inline]
fn sim(vec_a: &SpVec32, vec_b: &SpVec32, a_weight: f32) -> f32 {
    if !vec_a.could_overlap(vec_b) {
        return 0.0;
    }

    let sc = vec_a.scalar(vec_b);

    let ldiff = ((vec_a.get_length() * a_weight) + vec_b.get_length()) / (a_weight + 1.0);

    sc / ldiff
}
