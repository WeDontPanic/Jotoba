use autocompletion::{
    index::{
        japanese::{Item, JapaneseIndex},
        IndexItem,
    },
    relevance::{item::EngineItem, RelevanceCalc},
    suggest::{
        extension::{Extension, ExtensionOptions},
        query::SuggestionQuery,
    },
};
use japanese::JapaneseExt;
use priority_container::PrioContainerMax;

#[derive(Clone, Copy)]
pub struct KanaEndExtension<'a> {
    pub options: ExtensionOptions,
    index: &'a JapaneseIndex,
    max_dist: u32,
}

impl<'a> KanaEndExtension<'a> {
    /// Create a new Longest-Prefix Extension
    pub fn new(index: &'a JapaneseIndex, max_dist: u32) -> Self {
        let mut options = ExtensionOptions::default();
        options.weights.freq_weight = 0.01;
        Self {
            options,
            index,
            max_dist,
        }
    }
}

impl<'a> Extension<'a> for KanaEndExtension<'a> {
    #[inline]
    fn run(&self, query: &SuggestionQuery, rel_weight: f64) -> Vec<EngineItem<'a>> {
        let query_str = &query.query_str;

        let first_char = &query.query_str.chars().nth(0).unwrap();
        let last_char = &query.query_str.chars().last().unwrap();
        if !first_char.is_kanji() || !last_char.is_kana() {
            return vec![];
        }

        let mut parts: Vec<_> = japanese::text_parts(&query_str)
            .filter(|i| !i.trim().is_empty())
            .collect();
        if parts.len() != 2 {
            return vec![];
        }

        let kanji_part = parts.remove(0);
        let kana_part = parts.remove(0);
        let kana_hash = jpeudex::Hash::new(kana_part);

        let rel_weight = rel_weight * self.options.weights.total_weight;
        let mut out = PrioContainerMax::new(self.options.limit);

        let rel_calc = RelevanceCalc::new(self.options.weights).with_total_weight(rel_weight);

        let items = self.index.trie.iter_prefix_str(kanji_part);
        for j in items.map(|i| i.1).flatten() {
            let word = self.index.get_item(*j);
            if word.kanji.is_none() {
                continue;
            }

            let similarity = match word_similarity(word, kanji_part, kana_part, &kana_hash) {
                Some(s) => s,
                None => continue,
            };
            if similarity > self.max_dist {
                continue;
            }

            let mut item = word.into_engine_item();
            let str_rel = item.inner().str_relevance(&query.query_str);
            let rel = rel_calc.calc(&item, str_rel);
            item.set_relevance(rel);
            out.insert(item);
        }

        let out = out.into_iter().map(|i| i.0).collect::<Vec<_>>();
        let rel_calc = RelevanceCalc::new(self.options.weights).with_total_weight(rel_weight);
        query.order_items(out, rel_calc)
    }

    #[inline]
    fn should_run(&self, already_found: usize, _query: &SuggestionQuery) -> bool {
        self.options.enabled && already_found < self.options.threshold
    }

    #[inline]
    fn get_options(&self) -> &ExtensionOptions {
        &self.options
    }
}

#[inline]
fn word_similarity(
    item: &Item,
    kanji: &str,
    kana: &str,
    kana_hash: &Option<jpeudex::Hash>,
) -> Option<u32> {
    let item_kanji = item.kanji.as_ref().unwrap();
    if item.kana.ends_with(kana) && item_kanji.starts_with(kanji) {
        return Some(0);
    }

    if let Some(found_sub) = find_kana_str(&item.kana, kana) {
        let item_part = &item.kana[found_sub..];
        let l = item_part.chars().count();
        let kana_len = kana.chars().count();
        return Some((l - kana_len) as u32 * 2);
    }

    if let Some(kana_hash) = &kana_hash {
        let item_kana_hash = jpeudex::Hash::new(&item.kana)?;
        let dist = (item_kana_hash - *kana_hash).dist();
        return Some(dist);
    }

    None
}

/// Requires `full_kana` to be longer than `end_sub`
fn find_kana_str(full_kana: &str, end_sub: &str) -> Option<usize> {
    full_kana.match_indices(end_sub).last().map(|i| i.0)
}
