use super::REMOVE_PARENTHESES;
use engine::relevance::{data::SortData, RelevanceEngine};
use indexes::ng_freq::{vec_sim, NgFreqIndex};
use types::jotoba::{languages::Language, words::Word};
use vsm::{doc_vec::DocVector, Vector};

pub struct ForeignOrder;

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
        //let gloss_sim = query.scalar(index_item);
        let gloss_sim = query.scalar(index_item);
        let word = item.item();

        let lang = item.language().unwrap_or(Language::English);

        let q_vec = get_ng_index(lang).build_vec(&item.query_str().trim().to_lowercase());

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
                let vec = get_ng_index(lang).build_vec(&fmt);
                vec_sim(&vec, &q_vec)
            })
            .max_by(|a, b| a.total_cmp(&b))
            .unwrap_or(0.0);

        let mut rel_add = 0.0;
        if text_sim >= 0.6 {
            rel_add += gloss_sim * 100.0;
        }

        let score = (rel_add + text_sim) / 2.0;
        /* println!(
            "{}: ({rel_add} + {text_sim} ) / 2.0 = {score} [gloss_sim = {gloss_sim}]",
            word.get_reading_str()
        ); */
        score
    }
}

#[inline]
fn get_ng_index(lang: Language) -> &'static NgFreqIndex {
    indexes::get().word().foreign(lang).unwrap().ng_index()
}
