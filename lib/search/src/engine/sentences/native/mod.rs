pub mod index;

use crate::engine::{document::SentenceDocument, Indexable, SearchEngine};
use resources::storage::ResourceStorage;
use types::jotoba::{languages::Language, sentences::Sentence};
use vector_space_model2::{DefaultMetadata, Vector};

pub struct Engine {}

impl Indexable for Engine {
    type Metadata = DefaultMetadata;
    type Document = SentenceDocument;

    #[inline]
    fn get_index(
        _language: Option<Language>,
    ) -> Option<&'static vector_space_model2::Index<Self::Document, Self::Metadata>> {
        Some(index::get())
    }
}

impl SearchEngine for Engine {
    type Output = &'static Sentence;

    #[inline]
    fn doc_to_output(
        storage: &'static ResourceStorage,
        inp: &Self::Document,
    ) -> Option<Vec<Self::Output>> {
        storage.sentences().by_id(inp.seq_id).map(|i| vec![i])
    }

    fn gen_query_vector(
        index: &vector_space_model2::Index<Self::Document, Self::Metadata>,
        query: &str,
        _allow_align: bool,
        _language: Option<Language>,
    ) -> Option<(Vector, String)> {
        let mut terms = vec![query.to_string()];
        terms.extend(tinysegmenter::tokenize(query));
        //let vec = DocumentVector::new(index.get_indexer(), query_document.clone())?;
        //Some((vec, query.to_string()))
        let vec = index.build_vector(&terms, None)?;
        Some((vec, query.to_string()))
    }
}
