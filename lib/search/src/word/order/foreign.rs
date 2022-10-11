use super::REMOVE_PARENTHESES;
use engine::relevance::{data::SortData, RelevanceEngine};
use indexes::ng_freq::{term_dist, NgFreqIndex};
use sparse_vec::{SpVec32, VecExt};
use types::jotoba::{languages::Language, words::Word};
use vsm::doc_vec::DocVector;

pub struct ForeignOrder {
    query_vec_lang: SpVec32,
    query_vec_en: Option<SpVec32>,

    lang: Language,
}

impl ForeignOrder {
    #[inline]
    pub fn new() -> Self {
        Self {
            query_vec_lang: SpVec32::default(),
            query_vec_en: None,
            lang: Language::English,
        }
    }

    #[inline]
    fn get_query_vec(&self, lang: Language) -> &SpVec32 {
        if lang == self.lang {
            &self.query_vec_lang
        } else if lang == Language::English {
            self.query_vec_en.as_ref().unwrap()
        } else {
            log::error!("Unreachable");
            unreachable!()
        }
    }
}

impl RelevanceEngine for ForeignOrder {
    type OutItem = &'static Word;
    type IndexItem = DocVector<u32>;
    type Query = SpVec32;

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

        let q_vec = self.get_query_vec(lang);

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
                term_dist(&vec, q_vec)
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

        self.query_vec_lang = build_vec(get_ng_index(lang), &init.query);

        if lang != Language::English {
            self.query_vec_en = Some(build_vec(get_ng_index(Language::English), &init.query));
        }

        self.lang = lang;
    }
}

#[inline]
fn get_ng_index(lang: Language) -> &'static NgFreqIndex {
    indexes::get().word().foreign(lang).unwrap().ng_index()
}

#[inline]
pub fn build_vec(index: &NgFreqIndex, term: &str) -> SpVec32 {
    index.build_custom_vec(term, |freq, tot| freq / tot)
}
