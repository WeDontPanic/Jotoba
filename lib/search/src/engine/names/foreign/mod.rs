pub mod index;

use crate::engine::{document::MultiDocument, simple_gen_doc::GenDoc, Indexable, SearchEngine};
use resources::models::storage::ResourceStorage;
use types::jotoba::{languages::Language, names::Name};
use utils::to_option;
use vector_space_model::{DefaultMetadata, DocumentVector};

pub struct Engine {}

impl Indexable for Engine {
    type Metadata = DefaultMetadata;
    type Document = MultiDocument;

    #[inline]
    fn get_index(
        _language: Option<Language>,
    ) -> Option<&'static vector_space_model::Index<Self::Document, Self::Metadata>> {
        Some(index::get())
    }
}

impl SearchEngine for Engine {
    type GenDoc = GenDoc;
    type Output = Name;

    #[inline]
    fn doc_to_output(
        storage: &'static ResourceStorage,
        inp: &Self::Document,
    ) -> Option<Vec<&'static Self::Output>> {
        to_option(
            inp.seq_ids
                .iter()
                .map(|i| storage.names().by_sequence(*i).unwrap())
                .collect(),
        )
    }

    fn gen_query_vector(
        index: &vector_space_model::Index<Self::Document, Self::Metadata>,
        query: &str,
        _allow_align: bool,
        _language: Option<Language>,
    ) -> Option<(DocumentVector<Self::GenDoc>, String)> {
        let query_document = GenDoc::new(format_word(query));
        let vec = DocumentVector::new(index.get_indexer(), query_document.clone())?;
        Some((vec, query.to_string()))
    }

    fn align_query<'b>(
        original: &'b str,
        index: &vector_space_model::Index<Self::Document, Self::Metadata>,
        _language: Option<Language>,
    ) -> Option<&'b str> {
        let query_str = original;

        let mut indexer = index.get_indexer().clone();

        let has_term = indexer.find_term(&query_str).is_some()
            || indexer.find_term(&query_str.to_lowercase()).is_some();

        if has_term {
            return None;
        }

        let mut res = index::get_term_tree().find(&query_str.to_string(), 1);
        if res.is_empty() {
            res = index::get_term_tree().find(&query_str.to_string(), 2);
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
