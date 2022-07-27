use crate::engine::{search_task::sort_item::SortItem, Indexable, SearchEngine};
use indexes::names::ForeignIndex;
use ngindex::dice;
use types::jotoba::{languages::Language, names::Name};
use utils::to_option;
use vector_space_model2::{DefaultMetadata, Vector};

pub struct Engine {}

impl Indexable for Engine {
    type Metadata = DefaultMetadata;
    type Document = Vec<u32>;

    type Index = ForeignIndex;

    #[inline]
    fn get_index(_language: Option<Language>) -> Option<&'static Self::Index> {
        Some(indexes::get().name().foreign())
    }
}

impl SearchEngine for Engine {
    type Output = &'static Name;

    #[inline]
    fn doc_to_output(inp: &Self::Document) -> Option<Vec<Self::Output>> {
        let storage = resources::get();
        to_option(
            inp.iter()
                .map(|i| storage.names().by_sequence(*i).unwrap())
                .collect(),
        )
    }

    fn gen_query_vector(
        index: &Self::Index,
        query: &str,
        _allow_align: bool,
        _language: Option<Language>,
    ) -> Option<(Vector, String)> {
        let fmt = format_word(query);
        let query = index.build_vector(&[&fmt], None)?;
        Some((query, fmt))
    }

    #[inline]
    fn score(item: SortItem<Self::Output>) -> usize {
        let qvec = item.query_vec();
        let dvec = item.item_vec();
        (dice(qvec, dvec) * 100000.0) as usize
    }
}

/// Replaces all special characters into spaces so we can split it down into words
fn format_word(inp: &str) -> String {
    let mut out = String::from(inp);
    for i in ".,[]() \t\"'\\/-;:".chars() {
        out = out.replace(i, " ");
    }
    out.to_lowercase()
}
