use super::REMOVE_PARENTHESES;
use engine::relevance::{data::SortData, RelevanceEngine};
use indexes::ng_freq::{term_dist, NgFreqIndex};
use sparse_vec::{SpVec32, VecExt};
use types::jotoba::{
    language::{LangParam, Language},
    words::Word,
};
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
            // If `lang` is english and not the users lang, `query_vec_en` is always set
            self.query_vec_en.as_ref().unwrap()
        } else {
            // There are only search tasks for users language or english. So the query vector has
            // to be `query_vec_lang` in case `lang` is the users language, or `query_vec_en` if
            // the language is english. If there are other search requests, this code must be
            // adjusted
            log::error!("Unreachable");
            unreachable!()
        }
    }

    #[inline]
    fn text_sim(&self, word: &Word, lang: Language) -> f32 {
        let dist = |i: &str| -> f32 {
            let fmt = REMOVE_PARENTHESES.replace_all(i, "").trim().to_lowercase();
            if fmt.is_empty() {
                return 0.0;
            }
            let vec = build_vec(get_ng_index(lang), &fmt);
            term_dist(self.get_query_vec(lang), &vec)
        };

        word.gloss_iter_by_lang(LangParam::new(lang))
            .map(|i| dist(i))
            .chain(
                self.query_vec_en
                    .iter()
                    .map(|_| word.gloss_iter_by_lang(Language::English).map(|i| dist(i)))
                    .flatten(),
            )
            .max_by(|a, b| a.total_cmp(&b))
            .unwrap_or(0.0)
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
        let word = item.item();

        let lang = item.language().unwrap_or(Language::English);
        let text_sim = self.text_sim(word, lang);

        let mut rel_add = 0.0;
        if text_sim >= 0.5 {
            let index_item = item.index_item().vec();
            let gloss_sim = item.query().scalar(index_item);
            rel_add += gloss_sim * 100.0;
        }

        (rel_add + text_sim) / 2.0
    }

    fn init(&mut self, init: engine::relevance::RelEngineInit) {
        let lang = init.language.unwrap();

        let query = init.query.to_lowercase();
        self.query_vec_lang = build_vec(get_ng_index(lang), &query);

        if lang != Language::English {
            self.query_vec_en = Some(build_vec(get_ng_index(Language::English), &query));
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
    index.build_custom_vec(term, |_freq, _tot| 1.0)
}
