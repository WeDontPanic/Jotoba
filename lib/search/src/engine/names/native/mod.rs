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
        let query_document = GenDoc::new(vec![query]);
        let mut doc = DocumentVector::new(index.get_indexer(), query_document.clone())?;

        let terms = tinysegmenter::tokenize(query);
        doc.add_terms(index.get_indexer(), &terms, true, Some(0.4));

        Some((doc, query.to_owned()))
    }
}
