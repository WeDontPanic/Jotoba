pub mod index;

use resources::{
    models::{storage::ResourceStorage, words::Word},
    parse::jmdict::languages::Language,
};
use vector_space_model::{DefaultMetadata, DocumentVector};

use crate::{
    engine::{document::SingleDocument, simple_gen_doc::GenDoc},
    engine_v2::{Indexable, SearchEngine},
};

pub struct NativeEngine {
    //
}

impl Indexable for NativeEngine {
    type Metadata = DefaultMetadata;
    type Document = SingleDocument;

    #[inline]
    fn get_index(
        _language: Option<Language>,
    ) -> Option<&'static vector_space_model::Index<Self::Document, Self::Metadata>> {
        Some(index::get())
    }
}

impl SearchEngine for NativeEngine {
    type GenDoc = GenDoc;
    type Output = Word;

    #[inline]
    fn doc_to_output<'a>(
        storage: &'a ResourceStorage,
        inp: &Self::Document,
    ) -> Option<&'a Self::Output> {
        storage.words().by_sequence(inp.seq_id)
    }

    fn gen_query_vector(
        &self,
        index: &vector_space_model::Index<Self::Document, Self::Metadata>,
        query: &str,
    ) -> Option<DocumentVector<Self::GenDoc>> {
        let query_document = GenDoc::new(vec![query]);
        let mut doc = DocumentVector::new(index.get_indexer(), query_document.clone())?;

        // TODO: look if this makes the results really better. If not, remove
        let terms = tinysegmenter::tokenize(query);
        doc.add_terms(index.get_indexer(), &terms, true, Some(0.03));

        Some(doc)
    }
}
