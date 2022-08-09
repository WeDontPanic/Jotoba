use ngindex::{
    index_framework::retrieve::retriever::{ngram::NGramRetriever, Retriever},
    NgramIndex,
};
use qp_trie::{wrapper::BString, Trie};
use serde::{Deserialize, Serialize};
use types::jotoba::{indexes::hashtag::RawHashtag, search::SearchTarget};

/// Index for hashtag auto completion
#[derive(Deserialize, Serialize)]
pub struct HashTagIndex {
    tags: Vec<RawHashtag>,
    pub index: NgramIndex<2, u32>,
    trie: Trie<BString, u32>,
}

impl HashTagIndex {
    /// Create a new HashTagIndex
    pub fn new(tags: Vec<RawHashtag>, index: NgramIndex<2, u32>, trie: Trie<BString, u32>) -> Self {
        Self { tags, index, trie }
    }

    #[inline]
    pub fn get(&self, pos: usize) -> Option<&RawHashtag> {
        self.tags.get(pos)
    }

    #[inline]
    pub fn get_filtered(&self, pos: usize, s_targets: &[SearchTarget]) -> Option<&RawHashtag> {
        let tag = self.get(pos)?;
        if s_targets.iter().any(|i| tag.s_targets.contains(i)) || s_targets.is_empty() {
            return Some(tag);
        }
        None
    }

    #[inline]
    pub fn trie_search(&self, query: &str, s_targets: &[SearchTarget]) -> Vec<&RawHashtag> {
        let id = self.trie.subtrie_str(&query.to_lowercase());

        let mut out = vec![];
        for (_, id) in id.iter() {
            if let Some(v) = self.get_filtered(*id as usize, s_targets) {
                out.push(v);
            }
        }

        out
    }

    pub fn ngram_search(&self, query: &str, s_targets: &[SearchTarget]) -> Vec<(&RawHashtag, f32)> {
        let mut posts: Vec<_> = s_targets.iter().map(|i| i.get_type_id() as u32).collect();
        if posts.is_empty() {
            posts = vec![0, 1, 2, 3];
        }

        let retr = self
            .index
            .retriever_for(query)
            .in_postings(posts)
            .unique()
            .get::<NGramRetriever<'_, 2, _, _, _>>();

        let q = retr.q_term_ids().to_vec();

        let mut out = retr
            .filter_map(|i| {
                let item = self.get(*i.item() as usize)?;
                let dice = i.terms().dice_weighted(&q, 0.5);
                Some((item, dice))
            })
            .filter(|i| i.1 > 0.2)
            .collect::<Vec<_>>();
        out.sort_by(|a, b| a.1.total_cmp(&b.1).reverse());
        out
    }
}
