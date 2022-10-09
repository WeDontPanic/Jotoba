use std::collections::HashMap;

use super::REMOVE_PARENTHESES;
use engine::relevance::{data::SortData, RelevanceEngine};
use indexes::ng_freq::{vec_sim, NgFreqIndex};
use types::jotoba::{languages::Language, words::Word};
use vsm::{doc_vec::DocVector, Vector};

pub struct ForeignOrder {
    query_vecs: HashMap<Language, Vector>,
    lang: Language,
}

impl ForeignOrder {
    #[inline]
    pub fn new() -> Self {
        Self {
            query_vecs: HashMap::new(),
            lang: Language::English,
        }
    }
}

impl RelevanceEngine for ForeignOrder {
    type OutItem = &'static Word;
    type IndexItem = DocVector<u32>;
    type Query = Vector;

    #[inline]
    fn score<'item, 'query>(
        &self,
        item: &SortData<'item, 'query, Self::OutItem, Self::IndexItem, Self::Query>,
    ) -> f32 {
        let query = item.query();
        let index_item = item.index_item().vec();
        let gloss_sim = query.scalar(index_item);
        let word = item.item();

        let lang = item.language().unwrap_or(Language::English);
        let tindex = get_ng_index(lang);
        let q_vec = self.query_vecs.get(&lang).unwrap();

        //REMOVE_PARENTHESES.relpc
        let text_sim = word
            .gloss_iter_by_lang(lang, true)
            .map(|i| {
                let fmt = REMOVE_PARENTHESES.replace_all(i, "").trim().to_lowercase();
                if fmt.is_empty() {
                    /* println!(
                        "is empty: {i:?} -> {fmt:?} in word {}",
                        word.get_reading_str()
                    ); */
                    return 0.0;
                }
                let vec = build_vec(tindex, &fmt);
                vec_sim(&vec, &q_vec)
            })
            .max_by(|a, b| a.total_cmp(&b))
            .unwrap_or(0.0);

        let mut rel_add = 0.0;
        if text_sim >= 0.8 {
            rel_add += gloss_sim * 100.0;
        }

        let score = (rel_add + text_sim) / 2.0;
        /* println!(
            "{}: ({rel_add} + {text_sim} ) / 2.0 = {score} [gloss_sim = {gloss_sim}]",
            word.get_reading_str()
        ); */
        score
    }

    fn init(&mut self, init: engine::relevance::RelEngineInit) {
        let lang = init.language.unwrap();

        let qvec = build_vec(get_ng_index(lang), &init.query);
        self.query_vecs.insert(lang, qvec);
        assert!(self.query_vecs.contains_key(&lang));

        if lang != Language::English {
            let qvecen = build_vec(get_ng_index(Language::English), &init.query);
            self.query_vecs.insert(Language::English, qvecen);
        }

        self.lang = lang;
    }
}

#[inline]
fn get_ng_index(lang: Language) -> &'static NgFreqIndex {
    indexes::get().word().foreign(lang).unwrap().ng_index()
}

#[inline]
pub fn build_vec(index: &NgFreqIndex, term: &str) -> Vector {
    index.build_custom_vec(term, |freq, tot| freq / tot)
}
