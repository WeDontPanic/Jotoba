pub mod index;

use crate::{
    engine::{document::MultiDocument, simple_gen_doc::GenDoc},
    engine_v2::{Indexable, SearchEngine},
};
use resources::{
    models::{names::Name, storage::ResourceStorage},
    parse::jmdict::languages::Language,
};
use utils::to_option;
use vector_space_model::{DefaultMetadata, DocumentVector};

pub struct NativeEngine {}

impl Indexable for NativeEngine {
    type Metadata = DefaultMetadata;
    type Document = MultiDocument;

    #[inline]
    fn get_index(
        _language: Option<Language>,
    ) -> Option<&'static vector_space_model::Index<Self::Document, Self::Metadata>> {
        Some(index::get())
    }
}

impl SearchEngine for NativeEngine {
    type GenDoc = GenDoc;
    type Output = Name;

    #[inline]
    fn doc_to_output<'a>(
        storage: &'a ResourceStorage,
        inp: &Self::Document,
    ) -> Option<Vec<&'a Self::Output>> {
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
    ) -> Option<DocumentVector<Self::GenDoc>> {
        let query_document = GenDoc::new(vec![query]);
        let mut doc = DocumentVector::new(index.get_indexer(), query_document.clone())?;

        let terms = tinysegmenter::tokenize(query);
        doc.add_terms(index.get_indexer(), &terms, true, Some(0.4));

        Some(doc)
    }
}
