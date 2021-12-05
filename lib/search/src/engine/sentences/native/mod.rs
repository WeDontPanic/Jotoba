pub mod index;

use crate::engine::{document::SentenceDocument, simple_gen_doc::GenDoc, Indexable, SearchEngine};
use resources::models::storage::ResourceStorage;
use types::jotoba::{languages::Language, sentences::Sentence};
use vector_space_model::{DefaultMetadata, DocumentVector};

pub struct Engine {}

impl Indexable for Engine {
    type Metadata = DefaultMetadata;
    type Document = SentenceDocument;

    #[inline]
    fn get_index(
        _language: Option<Language>,
    ) -> Option<&'static vector_space_model::Index<Self::Document, Self::Metadata>> {
        Some(index::get())
    }
}

impl SearchEngine for Engine {
    type GenDoc = GenDoc;
    type Output = Sentence;

    #[inline]
    fn doc_to_output(
        storage: &'static ResourceStorage,
        inp: &Self::Document,
    ) -> Option<Vec<&'static Self::Output>> {
        storage.sentences().by_id(inp.seq_id).map(|i| vec![i])
    }

    fn gen_query_vector(
        index: &vector_space_model::Index<Self::Document, Self::Metadata>,
        query: &str,
        _allow_align: bool,
        _language: Option<Language>,
    ) -> Option<(DocumentVector<Self::GenDoc>, String)> {
        let terms = tinysegmenter::tokenize(query);
        let query_document = GenDoc::new(terms);
        let vec = DocumentVector::new(index.get_indexer(), query_document.clone())?;
        Some((vec, query.to_string()))
    }
}
