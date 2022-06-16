use crate::engine::{Indexable, SearchEngine};
use resources::storage::ResourceStorage;
use types::jotoba::{languages::Language, names::Name};
use utils::to_option;
use vector_space_model2::{DefaultMetadata, Vector};

pub struct Engine {}

impl Indexable for Engine {
    type Metadata = DefaultMetadata;
    type Document = Vec<u32>;

    #[inline]
    fn get_index(
        _language: Option<Language>,
    ) -> Option<&'static vector_space_model2::Index<Self::Document, Self::Metadata>> {
        Some(indexes::get().name().foreign())
    }
}

impl SearchEngine for Engine {
    type Output = &'static Name;

    #[inline]
    fn doc_to_output(
        storage: &'static ResourceStorage,
        inp: &Self::Document,
    ) -> Option<Vec<Self::Output>> {
        to_option(
            inp.iter()
                .map(|i| storage.names().by_sequence(*i).unwrap())
                .collect(),
        )
    }

    fn gen_query_vector(
        index: &vector_space_model2::Index<Self::Document, Self::Metadata>,
        query: &str,
        _allow_align: bool,
        _language: Option<Language>,
    ) -> Option<(Vector, String)> {
        let vec = index.build_vector(&format_word(query), None)?;
        Some((vec, query.to_string()))
    }

    fn align_query<'b>(
        original: &'b str,
        index: &vector_space_model2::Index<Self::Document, Self::Metadata>,
        _language: Option<Language>,
    ) -> Option<&'b str> {
        let query_str = original;

        let indexer = index.get_indexer();

        let has_term = indexer.find_term(&query_str).is_some()
            || indexer.find_term(&query_str.to_lowercase()).is_some();

        if has_term {
            return None;
        }

        let tree = indexes::get().name().term_tree();
        let mut res = tree.find(&query_str.to_string(), 1);

        if res.is_empty() {
            res = tree.find(&query_str.to_string(), 2);
        }
        res.sort_by(|a, b| a.1.cmp(&b.1));
        res.get(0).map(|i| i.0.as_str())
    }
}

/// Replaces all special characters into spaces so we can split it down into words
fn format_word(inp: &str) -> Vec<String> {
    let mut out = String::from(inp);
    for i in ".,[]() \t\"'\\/-;:".chars() {
        out = out.replace(i, " ");
    }
    out.split(' ').map(|i| i.to_lowercase()).collect::<Vec<_>>()
}
